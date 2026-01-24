use std::{
    collections::HashMap,
    io::{Read, Write},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::mpsc,
};

use postcard::{from_bytes, to_vec};
use transport::{DeviceConfig, HostTransport, TransResult, Widget, WidgetId};

use crate::{
    Host, TCP_ADDR, TCP_PACKET_LEN, TcpMessage, TcpMessageKind, TcpRequest, TcpResponse,
    TcpTransport,
};

impl TcpTransport<Host> {
    pub async fn new_host(
        mut request_handler: Box<dyn FnMut(TcpRequest) -> TcpResponse + Send>,
    ) -> Self {
        let stream = TcpStream::connect(TCP_ADDR)
            .await
            .expect("Device is not running! not able to connect!");
        let (response_tx, response_rx) = mpsc::unbounded_channel::<TcpResponse>();

        let (read, write) = stream.into_split();

        // Channel to send responses back to the writer
        let (tx, mut rx) = mpsc::unbounded_channel::<Vec<u8>>();

        let tx_clone = tx.clone();
        // Spawn reader task
        tokio::spawn(async move {
            let mut read = read;
            let tx = tx_clone;
            loop {
                let mut message = [0; TCP_PACKET_LEN];
                if read.read_exact(&mut message).await.is_err() {
                    break; // Connection closed
                }

                println!("captured message{message:?}");

                let len = message[0] as usize;

                let real_message = &message[1..len + 1];

                println!("Trying to deserialize: {:?}", real_message);

                // Parse and handle message
                let tcp_msg: TcpMessage =
                    from_bytes(&message[1..len + 1]).expect("should have deserialized properly");

                match tcp_msg.kind {
                    crate::TcpMessageKind::Request(req) => {
                        let resp = request_handler(req);
                        let tcp_message = TcpMessage::new(TcpMessageKind::Response(resp));
                        let bytes =
                            postcard::to_allocvec(&tcp_message).expect("should be serializable");
                        tx.send(bytes).expect("should not be full")
                    }

                    TcpMessageKind::Response(resp) => {
                        response_tx.send(resp);
                        // we need to be like hey! we have a response!
                        // so if any function is waiting it can be like :0 time to check it out
                    }
                }
            }
        });

        // Spawn writer task
        tokio::spawn(async move {
            let mut write = write;
            while let Some(msg) = rx.recv().await {
                let mut slice = [0; TCP_PACKET_LEN];

                slice[1..msg.len() + 1].copy_from_slice(&msg);

                slice[0] = msg.len() as u8;

                println!("sending bytes: {:#?}", slice);
                let _ = write.write_all(&slice).await;
            }
        });

        Self {
            writer: tx,
            responses: response_rx,
            // request_map: HashMap::new(),
            _kind: std::marker::PhantomData,
        }
    }

    fn send_message(&mut self, msg: TcpMessage) {
        let bytes = postcard::to_allocvec(&msg).expect("should be serializable");
        self.writer.send(bytes).expect("should not be full")
    }
}

impl HostTransport for TcpTransport<Host> {
    async fn set_widgets(&mut self, widget: Widget) -> TransResult<()> {
        todo!()
    }

    async fn get_widget(&mut self, id: WidgetId) -> TransResult<Widget> {
        todo!()
    }

    async fn get_battery_level(&mut self) -> TransResult<u8> {
        let message = TcpMessage::new(TcpMessageKind::Request(TcpRequest::GetBatteryLevel));

        println!("sending a message!");

        // let bytes = to_vec::<TcpMessage, 512>(&message).expect("should be serialized properly");

        self.send_message(message);

        if let Some(response) = self.responses.recv().await
            && let TcpResponse::RespondBatteryLevel(batt_level) = response
        {
            return Ok(batt_level);
        } else {
            panic!("unable to receive from device!!")
        };
    }

    async fn set_device_config(&mut self, config: DeviceConfig) -> TransResult<()> {
        todo!()
    }
}
