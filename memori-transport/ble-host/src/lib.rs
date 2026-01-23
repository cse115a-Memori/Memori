use transport::{DeviceConfig, HostTransport, TransResult, Widget, WidgetId};
use transport::ble_types::*;
use btleplug::api::{
    bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType, ValueNotification
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use tokio::time;
use futures::stream::StreamExt;

struct  HostBLETransport<'a> {
    adapter: &'a Adapter,
    device: Option<Peripheral>,

    // want to cache these since we can't directly access?
    // rx: Option<Characteristic>,
    // tx: Option<Characteristic>,
}

impl HostTransport for HostBLETransport<'_> {
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
