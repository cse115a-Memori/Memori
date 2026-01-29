use std::{collections::HashMap, sync::Arc};

use postcard::{from_bytes, to_allocvec};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpListener,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::{
        Mutex,
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    task::JoinHandle,
};
use tracing::{debug, error};
use transport::{TransError, TransResult};

pub use transport::DeviceTransport;

use crate::{
    DeviceRequest, DeviceResponse, DeviceTcpTransport, HostRequest, HostResponse, Message,
    MessageKind, Sequenced, TCP_ADDR, TcpTransportResult,
};

pub struct HostDisconnected {}

type Responses = Arc<Mutex<HashMap<u32, oneshot::Sender<HostResponse>>>>;
pub struct HostConnected {
    device_response_task: JoinHandle<()>,
    msg_sender: UnboundedSender<Message>,
    recv_task: JoinHandle<()>,
    responses: Responses,
    send_task: JoinHandle<()>,
    seq_num: u32,
}

impl Default for DeviceTcpTransport<HostDisconnected> {
    fn default() -> Self {
        DeviceTcpTransport::<HostDisconnected> {
            state: HostDisconnected {},
        }
    }
}

impl DeviceTcpTransport<HostDisconnected> {
    pub async fn connect(
        self,
    ) -> TcpTransportResult<(
        DeviceTcpTransport<HostConnected>,
        (
            UnboundedReceiver<Sequenced<HostRequest>>,
            UnboundedSender<Sequenced<DeviceResponse>>,
        ),
    )> {
        // initialize tcp listener
        let listener = TcpListener::bind(TCP_ADDR)
            .await
            .inspect_err(|e| error!("{:#?}", e))?;

        // form tcp stream
        let (stream, _) = listener
            .accept()
            .await
            .inspect_err(|e| error!("{:#?}", e))?;

        // split it up
        let (stream_rx, stream_tx) = stream.into_split();

        // channel for host requests
        let (host_request_tx, host_request_rx) =
            mpsc::unbounded_channel::<Sequenced<HostRequest>>();

        // channel for sending device responses
        let (device_response_tx, device_response_rx) =
            mpsc::unbounded_channel::<Sequenced<DeviceResponse>>();

        // channel for sending messages
        let (msg_sender_tx, msg_sender_rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
            mpsc::unbounded_channel::<Message>();

        // data structure to store responses
        let responses = Arc::new(Mutex::new(
            HashMap::<u32, oneshot::Sender<HostResponse>>::new(),
        ));

        // task to take responses and send it back out the wire
        let device_response_task = tokio::spawn(Self::resp_handler(
            msg_sender_tx.clone(),
            device_response_rx,
        ));

        // task to handle receiving messages from the other side of the wire
        let recv_task = tokio::spawn(Self::recv_handler(
            stream_rx,
            host_request_tx,
            responses.clone(),
        ));

        // task to send messages to the other side of the wire
        let send_task = tokio::spawn(Self::trans_handler(stream_tx, msg_sender_rx));

        Ok((
            DeviceTcpTransport::<HostConnected> {
                state: HostConnected {
                    msg_sender: msg_sender_tx,
                    responses,
                    send_task,
                    recv_task,
                    seq_num: 1,
                    device_response_task,
                },
            },
            (host_request_rx, device_response_tx),
        ))
    }

    /// Handler for sending responses from the device implementer into the sender task.
    ///
    ///**Warning**: This function should be called from a `tokio::spawn` as it will loop forever.
    async fn resp_handler(
        msg_sender_tx: UnboundedSender<Message>,
        mut device_response_rx: UnboundedReceiver<Sequenced<DeviceResponse>>,
    ) {
        while let Some(resp) = device_response_rx.recv().await {
            msg_sender_tx
                .send(resp.into())
                .expect("write channel should not be closed")
        }
    }

    /// Handler for receiving messages form the other side of the wire and doing the proper
    /// things with them.
    ///
    ///**Warning**: This function should be called from a `tokio::spawn` as it will loop forever.
    async fn recv_handler(
        mut stream_rx: OwnedReadHalf,
        host_request_tx: UnboundedSender<Sequenced<HostRequest>>,
        responses: Responses,
    ) {
        loop {
            let mut msg_len_buf = [0; size_of::<u32>()];
            if stream_rx
                .read_exact(&mut msg_len_buf)
                .await
                .inspect_err(|e| error!("{e:#?}"))
                .is_err()
            {
                error!("connection closed");
                break;
            }
            debug!("received header bytes: {msg_len_buf:?}");
            let msg_len = u32::from_be_bytes(msg_len_buf) as usize;

            let mut buf = vec![0u8; msg_len];
            if stream_rx.read_exact(&mut buf).await.is_err() {
                // connection closed
                error!("connection closed");
                break;
            }

            // now we try to deserialize this message
            debug!("received message_bytes: {buf:#?}");

            // this should only ever receive a device tcp request
            // actually it could be a device_tcp_request or a host tcp response
            let Ok(message): Result<Message, postcard::Error> =
                from_bytes(&buf).inspect_err(|e| error!("Failed to deserialize bytes {e:#?}"))
            else {
                continue;
            };

            debug!("received message: {message:#?}");

            let seq_num = message.seq_num;

            match message.kind {
                MessageKind::HostRequest(req) => {
                    host_request_tx
                        .send(Sequenced::new(seq_num, req))
                        .expect("this should not be closed");
                }

                MessageKind::HostResponse(resp) => {
                    let mut responses = responses.lock().await;
                    let tx = responses.remove(&seq_num ).expect("Invariant broken, expecting to have a oneshot channel to sent to for this response");
                    tx.send(resp).expect("receiver should not be closed");
                }

                _ => {
                    error!("Received invalid message type");
                    panic!("Received invalid message type");
                }
            }
        }
    }

    /// Handler that deals with sending any and all messages to the other side of the wire.
    ///
    ///**Warning**: This function should be called from a `tokio::spawn` as it will loop forever.
    async fn trans_handler(
        mut stream_tx: OwnedWriteHalf,
        mut msg_sender_rx: UnboundedReceiver<Message>,
    ) {
        while let Some(msg) = msg_sender_rx.recv().await {
            let msg_bytes = to_allocvec(&msg)
                .inspect_err(|e| error!("Failed to deserialize: {e:#?}"))
                .unwrap();

            let len = msg_bytes.len() as u32;
            let header_bytes = len.to_be_bytes();

            let message_bytes = &[&header_bytes[..], &msg_bytes].concat();

            debug!("sending message: {msg:#?}, bytes: {message_bytes:?}");

            stream_tx
                .write_all(&[&header_bytes[..], &msg_bytes].concat())
                .await
                .inspect_err(|e| error!("{e:#?}"))
                .unwrap()
        }
    }
}

impl DeviceTcpTransport<HostConnected> {
    /// Helper function to send requests to the other side of the wire,
    /// namely exists to deal with sequence numbers for requests and responses.
    async fn send_request(
        &mut self,
        msg: MessageKind,
    ) -> TransResult<oneshot::Receiver<HostResponse>> {
        self.state.seq_num = self.state.seq_num.saturating_add(2);

        let seq_num = self.state.seq_num;

        let msg = Message { seq_num, kind: msg };

        let mut responses = self.state.responses.lock().await;

        let (resp_tx, resp_rx) = oneshot::channel::<HostResponse>();
        responses.insert(seq_num, resp_tx);

        self.state.msg_sender.send(msg).map_err(|e| {
            error!("Failed to send into message sender! {e}");
            TransError::InternalError
        })?;

        Ok(resp_rx)
    }
}

impl DeviceTransport for DeviceTcpTransport<HostConnected> {
    async fn refresh_data(
        &mut self,
        widget_id: transport::WidgetId,
    ) -> transport::TransResult<transport::ByteArray> {
        let resp = self
            .send_request(MessageKind::DeviceRequest(DeviceRequest::RefreshData(
                widget_id,
            )))
            .await?
            .await
            .inspect_err(|e| error!("error receiving message: {e}"))
            .map_err(|_| TransError::InternalError)?;

        if let HostResponse::UpdatedWidget(data) = resp {
            Ok(*data)
        } else {
            panic!(
                "Invariant failed! the same seq_num had a different response type than the request"
            );
        }
    }

    async fn ping(&mut self) -> transport::TransResult<()> {
        let resp = self
            .send_request(MessageKind::DeviceRequest(DeviceRequest::Ping))
            .await?
            .await
            .inspect_err(|e| error!("error receiving message: {e}"))
            .map_err(|_| TransError::InternalError)?;

        if let HostResponse::Pong = resp {
            Ok(())
        } else {
            panic!(
                "Invariant failed! the same seq_num had a different response type than the request"
            );
        }
    }
}
impl DeviceTcpTransport<HostConnected> {
    pub fn disconnect(self) {
        // aborting the tasks so they dont run in the backgrund when transport is dropped
        self.state.send_task.abort();
        self.state.recv_task.abort();
        self.state.device_response_task.abort();
    }
}
