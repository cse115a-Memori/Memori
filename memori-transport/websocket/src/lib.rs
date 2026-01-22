/// Marker types for Host.
struct Host;
/// Marker type for Device.
struct Device;

const HOST_ADDR: &str = "ws://127.0.0.1:3000/ws";

pub mod device;

pub mod host;

struct WebsocketTransport<Kind> {
    _kind: std::marker::PhantomData<Kind>,
    // Add actual connection state here
    // connection: WebSocket,
}
