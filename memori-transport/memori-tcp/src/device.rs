use postcard::{from_bytes, to_allocvec};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tracing::{debug, error};
use transport::TransError;

pub use transport::DeviceTransport;

use crate::{
    DeviceRequest, DeviceResponse, DeviceTcpTransport, HostRequest, HostResponse, Message,
    TCP_ADDR, TcpTransportResult,
};

pub struct HostConnected {
    msg_sender: UnboundedSender<Message>,
    responses: UnboundedReceiver<HostResponse>,
    send_task: JoinHandle<()>,
    recv_task: JoinHandle<()>,
}

pub struct HostDisconnected {
    // request_handler: DeviceRequestHandler,
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
            UnboundedReceiver<HostRequest>,
            UnboundedSender<DeviceResponse>,
        ),
    )> {
        let listener = TcpListener::bind(TCP_ADDR)
            .await
            .inspect_err(|e| error!("{:#?}", e))?;
        let (stream, _) = listener
            .accept()
            .await
            .inspect_err(|e| error!("{:#?}", e))?;

        let (stream_rx, stream_tx) = stream.into_split();

        let (host_request_tx, host_request_rx) = mpsc::unbounded_channel::<HostRequest>();
        let (device_response_tx, mut device_response_rx) =
            mpsc::unbounded_channel::<DeviceResponse>();

        let (write_tx, mut write_rx) = mpsc::unbounded_channel::<Message>();

        // to send responses out of the reader task
        let (response_tx, response_rx) = mpsc::unbounded_channel::<HostResponse>();

        let write_tx_clone = write_tx.clone();

        let device_response_task = tokio::spawn(async move {
            let write_tx = write_tx_clone;
            if let Some(resp) = device_response_rx.recv().await {
                let message = Message::DeviceResponse(Box::new(resp));
                write_tx
                    .send(message)
                    .expect("write channel should not be closed")
            }
        });

        let recv_task = tokio::spawn(async move {
            let mut stream_rx = stream_rx;
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

                if msg_len > 2048 {
                    error!("received message that's longer than 2048 bytes, aborting message");
                    continue;
                }

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
                let Ok(message) =
                    from_bytes(&buf).inspect_err(|e| error!("Failed to deserialize bytes {e:#?}"))
                else {
                    continue;
                };

                debug!("received message: {message:#?}");

                match message {
                    Message::HostRequest(req) => {
                        // we send the host request, if it fails, its because it closed, we cant really do anything about that.
                        let _ = host_request_tx.send(req);
                    }

                    Message::HostResponse(resp) => response_tx
                        .send(resp)
                        .inspect_err(|e| error!("response_tx channel closed: {e:?}"))
                        .unwrap(),

                    _ => {
                        error!("Received invalid message type");
                        panic!("Received invalid message type");
                    }
                }
            }
        });

        let send_task = tokio::spawn(async move {
            let mut stream_tx = stream_tx;

            while let Some(msg) = write_rx.recv().await {
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
        });

        Ok((
            DeviceTcpTransport::<HostConnected> {
                state: HostConnected {
                    msg_sender: write_tx,
                    responses: response_rx,
                    send_task,
                    recv_task,
                },
            },
            (host_request_rx, device_response_tx),
        ))
    }
}

impl DeviceTransport for DeviceTcpTransport<HostConnected> {
    async fn refresh_data(
        &mut self,
        widget_id: transport::WidgetId,
    ) -> transport::TransResult<transport::ByteArray> {
        self.state
            .msg_sender
            .send(Message::DeviceRequest(DeviceRequest::RefreshData(
                widget_id,
            )))
            .map_err(|e| {
                error!("Failed to send into message sender! {e:#?}");
                TransError::InternalError
            })?;

        if let Some(resp) = self.state.responses.recv().await
            && let HostResponse::UpdatedWidget(data) = resp
        {
            Ok(*data)
        } else {
            error!("Failed to receive the expected response");
            Err(TransError::NoAck)
        }
    }

    async fn ping(&mut self) -> transport::TransResult<()> {
        self.state
            .msg_sender
            .send(Message::DeviceRequest(DeviceRequest::Ping))
            .map_err(|e| {
                error!("Failed to send into message sender! {e:#?}");
                TransError::InternalError
            })?;

        if let Some(resp) = self.state.responses.recv().await
            && let HostResponse::Pong = resp
        {
            Ok(())
        } else {
            error!("Failed to receive the expected response");
            Err(TransError::NoAck)
        }
    }
}
impl DeviceTcpTransport<HostConnected> {
    pub fn disconnect(self) {
        // aborting the tasks so they dont run in the backgrund when transport is dropped
        self.state.send_task.abort();
        self.state.recv_task.abort();
    }
}
