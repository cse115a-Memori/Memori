use std::{collections::HashMap, io::Write, sync::Arc};

use postcard::from_bytes;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::{Mutex, mpsc::UnboundedReceiver},
};

use transport::{ByteArray, DeviceTransport, TransResult, WidgetId};

use crate::{Device, TCP_ADDR, TcpMessage, TcpMessageKind, TcpRequest, TcpResponse, TcpTransport};

impl DeviceTransport for TcpTransport<Device> {
    async fn ping(&mut self) -> TransResult<()> {
        todo!()
    }

    async fn refresh_data(&mut self, widget_id: WidgetId) -> TransResult<ByteArray> {
        todo!()
    }
}

use tokio::sync::mpsc;

impl TcpTransport<Device> {
    pub async fn new_device(
        mut request_handler: Box<dyn FnMut(TcpRequest) -> TcpResponse + Send>,
    ) -> Self {
        let listener = TcpListener::bind(TCP_ADDR).await.expect("bind failed");
        let (stream, _) = listener.accept().await.expect("accept failed");
        let (read, write) = stream.into_split();
        let (response_tx, response_rx) = mpsc::unbounded_channel::<TcpResponse>();

        // Channel to send responses back to the writer
        let (tx, mut rx) = mpsc::unbounded_channel::<Vec<u8>>();

        let tx_clone = tx.clone();
        // Spawn reader task
        tokio::spawn(async move {
            let mut read = read;
            let tx = tx_clone;
            loop {
                let mut message = [0; 512];
                if read.read_exact(&mut message).await.is_err() {
                    break; // Connection closed
                }

                // Parse and handle message
                let tcp_msg: TcpMessage =
                    from_bytes(&message).expect("should have deserialized properly");

                match tcp_msg.kind {
                    crate::TcpMessageKind::Request(req) => {
                        let resp = request_handler(req);
                        let bytes = postcard::to_allocvec(&resp).expect("should be serializable");
                        tx.send(bytes).expect("should not be full")
                    }

                    TcpMessageKind::Response(resp) => {
                        response_tx.send(resp).expect("should work properly");
                    }
                }
            }
        });

        // Spawn writer task
        tokio::spawn(async move {
            let mut write = write;
            while let Some(msg) = rx.recv().await {
                let mut slice = [0; 512];

                slice[0..msg.len()].copy_from_slice(&msg);

                println!("sending bytes: {:#?}", slice);
                let _ = write.write_all(&slice).await;
            }
        });

        Self {
            writer: tx,
            responses: response_rx,
            request_map: HashMap::new(),
            _kind: std::marker::PhantomData,
        }
    }

    fn send_message(&mut self, msg: TcpRequest) {
        let bytes = postcard::to_allocvec(&msg).expect("should be serializable");
        self.writer.send(bytes).expect("should not be full")
    }
}
