use transport::{DeviceConfig, HostTransport, TransResult, Widget, WidgetId};

use crate::{Host, WebsocketTransport};

impl WebsocketTransport<Host> {
    pub fn new_host(/* connection params */) -> Self {
        // would need to start server at the server address
        Self {
            _kind: std::marker::PhantomData,
        }
    }
}

impl HostTransport for WebsocketTransport<Host> {
    async fn set_widgets(&mut self, widget: Widget) -> TransResult<()> {
        todo!()
    }

    async fn get_widget(&mut self, id: WidgetId) -> TransResult<Widget> {
        todo!()
    }

    async fn get_battery_level(&mut self) -> TransResult<u8> {
        todo!()
    }

    async fn set_device_config(&mut self, config: DeviceConfig) -> TransResult<()> {
        todo!()
    }
}
