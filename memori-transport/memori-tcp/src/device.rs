use std::pin::Pin;

use postcard::{from_bytes, to_allocvec};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::mpsc,
};
use tracing::{debug, error};
use transport::DeviceTransport;

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
        let listener = TcpListener::bind(TCP_ADDR).await.expect("bind failed");
        let (stream, _) = listener.accept().await.expect("accept failed");

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

                    // signifies that connection closed
                    break;
                }
                debug!("received header bytes: {msg_len_buf:?}");
                let msg_len = u32::from_be_bytes(msg_len_buf) as usize;
                let mut buf = vec![0u8; msg_len];
                if stream_rx.read_exact(&mut buf).await.is_err() {
                    // connection closed
                    break;
                }

                // now we try to deserialize this message
                debug!("received message_bytes: {buf:#?}");

                // this should only ever receive a device tcp request
                // actually it could be a device_tcp_request or a host tcp response
                let message: Message = from_bytes(&buf).expect("Must deserialize properly");

                debug!("received message: {message:#?}");

                match message {
                    Message::HostRequest(req) => {
                        let resp = request_handler(req).await;

                        write_tx
                            .send(Message::DeviceResponse(Box::new(resp)))
                            .expect("should not be closed");
                    }

                    Message::HostResponse(resp) => {
                        response_tx.send(resp).expect("should not be closed")
                    }

                    _ => {
                        panic!("should never be possible")
                    }
                }
            }
        });

        let send_task = tokio::spawn(async move {
            let mut stream_tx = stream_tx;

            while let Some(msg) = write_rx.recv().await {
                let msg_bytes = to_allocvec(&msg).expect("must be serialized properly");

                let len = msg_bytes.len() as u32;
                let header_bytes = len.to_be_bytes();

                let message_bytes = &[&header_bytes[..], &msg_bytes].concat();

                debug!("sending message bytes: {message_bytes:?}");

                stream_tx
                    .write_all(&[&header_bytes[..], &msg_bytes].concat())
                    .await
                    .expect("Failed to write properly!");
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
            .expect("please fix this error handling");

        if let Some(resp) = self.responses.recv().await
            && let HostResponse::UpdatedWidget(data) = resp
        {
            Ok(*data)
        } else {
            panic!("fix this error handling dawg");
        }
    }

    async fn ping(&mut self) -> transport::TransResult<()> {
        self.msg_sender
            .send(Message::DeviceRequest(DeviceRequest::Ping))
            .expect("please fix this error handling");

        if let Some(resp) = self.responses.recv().await
            && let HostResponse::Pong = resp
        {
            Ok(())
        } else {
            panic!("fix this error handling dawg");
        }
    }
}
impl Drop for DeviceTcpTransport {
    fn drop(&mut self) {
        self.send_task.abort();
        self.recv_task.abort();
    }
}
