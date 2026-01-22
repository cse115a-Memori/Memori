use transport::{
    ByteArray, DeviceConfig, DeviceTransport, HostTransport, TransResult, Widget, WidgetId,
};

// Marker types for type-state pattern
struct Host;
struct Device;

const HOST_ADDR: &str = "ws://127.0.0.1:3000/ws";

struct WebsocketTransport<Kind> {
    _kind: std::marker::PhantomData<Kind>,
    // Add actual connection state here
    // connection: WebSocket,
}

impl HostTransport for WebsocketTransport<Host> {
    fn set_widgets(&mut self, widget: Widget) -> TransResult<()> {
        todo!()
    }

    fn get_widget(&mut self, id: WidgetId) -> TransResult<Widget> {
        todo!()
    }

    fn get_battery_level(&mut self) -> TransResult<u8> {
        todo!()
    }

    fn set_device_config(&mut self, config: DeviceConfig) -> TransResult<()> {
        todo!()
    }
}

impl DeviceTransport for WebsocketTransport<Device> {
    fn ping(&mut self) -> TransResult<()> {
        todo!()
    }

    fn refresh_data(&mut self, widget_id: WidgetId) -> TransResult<ByteArray> {
        todo!()
    }
}

// Separate constructors for each type
impl WebsocketTransport<Host> {
    pub fn new_host(/* connection params */) -> Self {
        // would need to start server at the server address
        Self {
            _kind: std::marker::PhantomData,
        }
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
