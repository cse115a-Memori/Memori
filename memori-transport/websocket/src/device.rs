use transport::{ByteArray, DeviceTransport, TransResult, WidgetId};

use crate::{Device, WebsocketTransport};

impl DeviceTransport for WebsocketTransport<Device> {
    async fn ping(&mut self) -> TransResult<()> {
        todo!()
    }

    async fn refresh_data(&mut self, widget_id: WidgetId) -> TransResult<ByteArray> {
        todo!()
    }
}

impl WebsocketTransport<Device> {
    pub fn new_device(/* connection params */) -> Self {
        // would need to connect to the server at the server address
        Self {
            _kind: std::marker::PhantomData,
        }
    }
}
