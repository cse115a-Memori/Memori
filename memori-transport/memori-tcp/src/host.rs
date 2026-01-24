use std::pin::Pin;
use tracing::{debug, error};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
};

use postcard::{from_bytes, to_allocvec};
use transport::HostTransport;

use crate::{
    DeviceRequest, DeviceResponse, HostResponse, HostTcpTransport, Message, TCP_ADDR,
    TcpTransportError, TcpTransportResult,
};

pub type HostRequestHandler =
    Box<dyn FnMut(DeviceRequest) -> Pin<Box<dyn Future<Output = HostResponse> + Send>> + Send>;

impl HostTcpTransport {
    /// `request_handler` is just an async function that takes in a `DeviceRequst` and returns a `HostResponse`
    pub async fn new<F, Fut>(mut request_handler: F) -> TcpTransportResult<Self>
    where
        F: FnMut(DeviceRequest) -> Fut + Send + 'static,
        Fut: Future<Output = HostResponse> + Send + 'static,
    {
        let stream = TcpStream::connect(TCP_ADDR)
            .await
            .map_err(|e| TcpTransportError::ConnectionError(e.to_string()))?;

        let (stream_rx, stream_tx) = stream.into_split();

        let (write_tx, mut write_rx) = mpsc::unbounded_channel::<Message>();

        // to send responses out of the reader task
        let (response_tx, response_rx) = mpsc::unbounded_channel::<DeviceResponse>();

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

                let message: Message = from_bytes(&buf)
                    .inspect_err(|e| error!("Failed to deserialize message: {e:?}"))
                    .expect("Must deserialize properly");
                match message {
                    Message::DeviceRequest(req) => {
                        let resp = request_handler(req).await;
                        write_tx
                            .send(Message::HostResponse(resp))
                            .inspect_err(|e| error!("write_tx channel closed: {e:?}"))
                            .unwrap();
                    }
                    Message::DeviceResponse(resp) => {
                        response_tx
                            .send(*resp)
                            .inspect_err(|e| error!("response_tx channel closed: {e:?}"))
                            .unwrap();
                    }
                    _ => {
                        error!("Received invalid message type");
                        panic!("should never be possible");
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
                    .write_all(message_bytes)
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

impl HostTransport for HostTcpTransport {
    async fn set_widgets(&mut self, widget: transport::Widget) -> transport::TransResult<()> {
        self.msg_sender
            .send(Message::HostRequest(crate::HostRequest::SetWidgets(
                Box::new(widget),
            )))
            .expect("please fix this error handling");

        if let Some(resp) = self.responses.recv().await
            && let DeviceResponse::Success = resp
        {
            Ok(())
        } else {
            panic!("Fix this error handling")
        }
    }

    async fn get_widget(
        &mut self,
        id: transport::WidgetId,
    ) -> transport::TransResult<transport::Widget> {
        self.msg_sender
            .send(Message::HostRequest(crate::HostRequest::GetWidget(id)))
            .expect("please fix this error handling");

        if let Some(resp) = self.responses.recv().await
            && let DeviceResponse::Widget(widget) = resp
        {
            Ok(*widget)
        } else {
            panic!("Fix this error handling")
        }
    }

    async fn get_battery_level(&mut self) -> transport::TransResult<u8> {
        self.msg_sender
            .send(Message::HostRequest(crate::HostRequest::GetBatteryLevel))
            .expect("please fix this error handling");

        if let Some(resp) = self.responses.recv().await
            && let DeviceResponse::BatteryLevel(level) = resp
        {
            Ok(level)
        } else {
            panic!("Fix this error handling")
        }
    }

    async fn set_device_config(
        &mut self,
        config: transport::DeviceConfig,
    ) -> transport::TransResult<()> {
        self.msg_sender
            .send(Message::HostRequest(crate::HostRequest::SetDeviceConfig(
                config,
            )))
            .expect("please fix this error handling");

        if let Some(resp) = self.responses.recv().await
            && let DeviceResponse::Success = resp
        {
            Ok(())
        } else {
            panic!("Fix this error handling")
        }
    }
}

impl Drop for HostTcpTransport {
    fn drop(&mut self) {
        self.send_task.abort();
        self.recv_task.abort();
    }
}
