use std::pin::Pin;

use postcard::{from_bytes, to_allocvec};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::mpsc,
};
use tracing::{debug, error};
use transport::{DeviceTransport, TransError};

use crate::{
    DeviceRequest, DeviceResponse, DeviceTcpTransport, HostRequest, HostResponse, Message,
    TCP_ADDR, TcpTransportResult,
};

pub type DeviceRequestHandler =
    Box<dyn FnMut(HostRequest) -> Pin<Box<dyn Future<Output = DeviceResponse> + Send>> + Send>;

impl DeviceTcpTransport {
    /// `request_handler` is just an async function that takes in a `DeviceRequst` and returns a `HostResponse`
    pub async fn new<F, Fut>(mut request_handler: F) -> TcpTransportResult<Self>
    where
        F: FnMut(HostRequest) -> Fut + Send + 'static,
        Fut: Future<Output = DeviceResponse> + Send + 'static,
    {
        let listener = TcpListener::bind(TCP_ADDR)
            .await
            .inspect_err(|e| error!("{:#?}", e))?;
        let (stream, _) = listener
            .accept()
            .await
            .inspect_err(|e| error!("{:#?}", e))?;

        let (stream_rx, stream_tx) = stream.into_split();

        let (write_tx, mut write_rx) = mpsc::unbounded_channel::<Message>();

        // to send responses out of the reader task
        let (response_tx, response_rx) = mpsc::unbounded_channel::<HostResponse>();

        let write_tx_clone = write_tx.clone();

        let recv_task = tokio::spawn(async move {
            let mut stream_rx = stream_rx;
            let write_tx = write_tx_clone;

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
                let message: Message = from_bytes(&buf)
                    .inspect_err(|e| error!("Failed to deserialize bytes {e:#?}"))
                    .unwrap();

                debug!("received message: {message:#?}");

                match message {
                    Message::HostRequest(req) => {
                        let resp = request_handler(req).await;

                        write_tx
                            .send(Message::DeviceResponse(Box::new(resp)))
                            .inspect_err(|e| error!("write_tx channel closed: {e:?}"))
                            .unwrap();
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

                debug!("sending message bytes: {message_bytes:?}");

                stream_tx
                    .write_all(&[&header_bytes[..], &msg_bytes].concat())
                    .await
                    .inspect_err(|e| error!("{e:#?}"))
                    .unwrap()
            }
        });

        Ok(Self {
            msg_sender: write_tx,
            responses: response_rx,
            send_task,
            recv_task,
        })
    }
}

impl DeviceTransport for DeviceTcpTransport {
    async fn refresh_data(
        &mut self,
        widget_id: transport::WidgetId,
    ) -> transport::TransResult<transport::ByteArray> {
        self.msg_sender
            .send(Message::DeviceRequest(DeviceRequest::RefreshData(
                widget_id,
            )))
            .map_err(|e| {
                error!("Failed to send into message sender! {e:#?}");
                TransError::InternalError
            })?;

        if let Some(resp) = self.responses.recv().await
            && let HostResponse::UpdatedWidget(data) = resp
        {
            Ok(*data)
        } else {
            error!("Failed to receive the expected response");
            Err(TransError::NoAck)
        }
    }

    async fn ping(&mut self) -> transport::TransResult<()> {
        self.msg_sender
            .send(Message::DeviceRequest(DeviceRequest::Ping))
            .map_err(|e| {
                error!("Failed to send into message sender! {e:#?}");
                TransError::InternalError
            })?;

        if let Some(resp) = self.responses.recv().await
            && let HostResponse::Pong = resp
        {
            Ok(())
        } else {
            error!("Failed to receive the expected response");
            Err(TransError::NoAck)
        }
    }
}
impl Drop for DeviceTcpTransport {
    fn drop(&mut self) {
        self.send_task.abort();
        self.recv_task.abort();
    }
}
