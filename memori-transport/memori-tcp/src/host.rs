use std::pin::Pin;
use tracing::{debug, error};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use postcard::{from_bytes, to_allocvec};
use transport::{HostTransport, TransError};

use crate::{
    DeviceRequest, DeviceResponse, HostResponse, HostTcpTransport, Message, TCP_ADDR,
    TcpTransportResult,
};

pub type HostRequestHandler =
    Box<dyn FnMut(DeviceRequest) -> Pin<Box<dyn Future<Output = HostResponse> + Send>> + Send>;

pub struct DeviceDisconnected {
    pub request_handler: HostRequestHandler,
}

pub struct DeviceConnected {
    msg_sender: UnboundedSender<Message>,
    responses: UnboundedReceiver<DeviceResponse>,
    send_task: JoinHandle<()>,
    recv_task: JoinHandle<()>,
}

impl HostTcpTransport<DeviceDisconnected> {
    pub fn new<F, Fut>(mut request_handler: F) -> Self
    where
        F: FnMut(DeviceRequest) -> Fut + Send + 'static,
        Fut: Future<Output = HostResponse> + Send + 'static,
    {
        Self {
            state: DeviceDisconnected {
                request_handler: Box::new(move |req| Box::pin(request_handler(req))),
            },
        }
    }

    pub async fn connect(self) -> TcpTransportResult<HostTcpTransport<DeviceConnected>> {
        let stream = TcpStream::connect(TCP_ADDR).await?;

        let mut request_handler = self.state.request_handler;

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

        Ok(HostTcpTransport {
            state: DeviceConnected {
                msg_sender: write_tx,
                responses: response_rx,
                send_task,
                recv_task,
            },
        })
    }
}

// impl HostTcpTransport {
//     /// `request_handler` is just an async function that takes in a `DeviceRequst` and returns a `HostResponse`
//     pub async fn new<F, Fut>(mut request_handler: F) -> TcpTransportResult<Self>
//     where
//         F: FnMut(DeviceRequest) -> Fut + Send + 'static,
//         Fut: Future<Output = HostResponse> + Send + 'static,
//     {
//         let stream = TcpStream::connect(TCP_ADDR).await?;

//         let (stream_rx, stream_tx) = stream.into_split();

//         let (write_tx, mut write_rx) = mpsc::unbounded_channel::<Message>();

//         // to send responses out of the reader task
//         let (response_tx, response_rx) = mpsc::unbounded_channel::<DeviceResponse>();

//         let write_tx_clone = write_tx.clone();
//         let recv_task = tokio::spawn(async move {
//             let mut stream_rx = stream_rx;
//             let write_tx = write_tx_clone;
//             loop {
//                 let mut msg_len_buf = [0; size_of::<u32>()];
//                 if stream_rx
//                     .read_exact(&mut msg_len_buf)
//                     .await
//                     .inspect_err(|e| error!("{e:#?}"))
//                     .is_err()
//                 {
//                     error!("connection closed");
//                     break;
//                 }
//                 debug!("received header bytes: {msg_len_buf:?}");
//                 let msg_len = u32::from_be_bytes(msg_len_buf) as usize;
//                 let mut buf = vec![0u8; msg_len];
//                 if stream_rx.read_exact(&mut buf).await.is_err() {
//                     // connection closed
//                     error!("connection closed");
//                     break;
//                 }

//                 // now we try to deserialize this message
//                 debug!("received message_bytes: {buf:#?}");

//                 // this should only ever receive a device tcp request
//                 // actually it could be a device_tcp_request or a host tcp response
//                 let message: Message = from_bytes(&buf)
//                     .inspect_err(|e| error!("Failed to deserialize bytes {e:#?}"))
//                     .unwrap();

//                 debug!("received message: {message:#?}");

//                 let message: Message = from_bytes(&buf)
//                     .inspect_err(|e| error!("Failed to deserialize message: {e:?}"))
//                     .expect("Must deserialize properly");

//                 match message {
//                     Message::DeviceRequest(req) => {
//                         let resp = request_handler(req).await;
//                         write_tx
//                             .send(Message::HostResponse(resp))
//                             .inspect_err(|e| error!("write_tx channel closed: {e:?}"))
//                             .unwrap();
//                     }
//                     Message::DeviceResponse(resp) => {
//                         response_tx
//                             .send(*resp)
//                             .inspect_err(|e| error!("response_tx channel closed: {e:?}"))
//                             .unwrap();
//                     }
//                     _ => {
//                         error!("Received invalid message type");
//                         panic!("Received invalid message type");
//                     }
//                 }
//             }
//         });

//         let send_task = tokio::spawn(async move {
//             let mut stream_tx = stream_tx;

//             while let Some(msg) = write_rx.recv().await {
//                 let msg_bytes = to_allocvec(&msg)
//                     .inspect_err(|e| error!("Failed to deserialize: {e:#?}"))
//                     .unwrap();

//                 let len = msg_bytes.len() as u32;
//                 let header_bytes = len.to_be_bytes();

//                 let message_bytes = &[&header_bytes[..], &msg_bytes].concat();

//                 debug!("sending message bytes: {message_bytes:?}");

//                 stream_tx
//                     .write_all(&[&header_bytes[..], &msg_bytes].concat())
//                     .await
//                     .inspect_err(|e| error!("{e:#?}"))
//                     .unwrap()
//             }
//         });

//         Ok(Self {
//             msg_sender: write_tx,
//             responses: response_rx,
//             send_task,
//             recv_task,
//         })
//     }
// }

impl HostTransport for HostTcpTransport<DeviceConnected> {
    async fn set_widgets(&mut self, widget: transport::Widget) -> transport::TransResult<()> {
        self.state
            .msg_sender
            .send(Message::HostRequest(crate::HostRequest::SetWidgets(
                Box::new(widget),
            )))
            .map_err(|e| {
                error!("Failed to send into message sender! {e:#?}");
                TransError::InternalError
            })?;

        if let Some(resp) = self.state.responses.recv().await
            && let DeviceResponse::Success = resp
        {
            Ok(())
        } else {
            error!("Failed to receive the expected response");
            Err(TransError::NoAck)
        }
    }

    async fn get_widget(
        &mut self,
        id: transport::WidgetId,
    ) -> transport::TransResult<transport::Widget> {
        self.state
            .msg_sender
            .send(Message::HostRequest(crate::HostRequest::GetWidget(id)))
            .map_err(|e| {
                error!("Failed to send into message sender! {e:#?}");
                TransError::InternalError
            })?;

        if let Some(resp) = self.state.responses.recv().await
            && let DeviceResponse::Widget(widget) = resp
        {
            Ok(*widget)
        } else {
            error!("Failed to receive the expected response");
            Err(TransError::NoAck)
        }
    }

    async fn get_battery_level(&mut self) -> transport::TransResult<u8> {
        self.state
            .msg_sender
            .send(Message::HostRequest(crate::HostRequest::GetBatteryLevel))
            .map_err(|e| {
                error!("Failed to send into message sender! {e:#?}");
                TransError::InternalError
            })?;

        if let Some(resp) = self.state.responses.recv().await
            && let DeviceResponse::BatteryLevel(level) = resp
        {
            Ok(level)
        } else {
            error!("Failed to receive the expected response");
            Err(TransError::NoAck)
        }
    }

    async fn set_device_config(
        &mut self,
        config: transport::DeviceConfig,
    ) -> transport::TransResult<()> {
        self.state
            .msg_sender
            .send(Message::HostRequest(crate::HostRequest::SetDeviceConfig(
                config,
            )))
            .map_err(|e| {
                error!("Failed to send into message sender! {e:#?}");
                TransError::InternalError
            })?;

        if let Some(resp) = self.state.responses.recv().await
            && let DeviceResponse::Success = resp
        {
            Ok(())
        } else {
            error!("Failed to receive the expected response");
            Err(TransError::NoAck)
        }
    }
}

impl HostTcpTransport<DeviceConnected> {
    pub fn disconnect(self) {
        // aborting the tasks so they dont run in the backgrund when transport is dropped
        self.state.send_task.abort();
        self.state.recv_task.abort();
    }
}
