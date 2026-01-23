use transport::{ByteArray, DeviceTransport, TransResult, WidgetId, TransError};
use transport::ble_types::*;
use trouble_host::prelude::*;
use log::{error, warn, info, debug, trace};

const CHARACTERISTIC_SIZE: usize = 32;
// GATT Server definition
//
#[gatt_server]
struct Server {
    nus_service: NordicUartService,
}

#[gatt_service(uuid = "6e400001-b5a3-f393-e0a9-e50e24dcca9e")]
struct NordicUartService {
    #[characteristic(uuid = "6e400002-b5a3-f393-e0a9-e50e24dcca9e", write, write_without_response)]
    #[descriptor(uuid = "2901", value = "RX Characteristic")]
    rx: [u8; CHARACTERISTIC_SIZE],
    
    #[characteristic(uuid = "6e400003-b5a3-f393-e0a9-e50e24dcca9e", read, notify)]
    #[descriptor(uuid = "2901", value = "TX Characteristic")]
    tx: [u8; CHARACTERISTIC_SIZE],
}

struct DeviceBLETransport<'a, P: PacketPool> {
    connection: &'a GattConnection<'a, 'a, P>,
    gattServer: &'a Server<'a>,
}

impl<'a, P: PacketPool> DeviceTransport for DeviceBLETransport<'a, P> {
    async fn ping(&mut self) -> TransResult<()> {
        let tx = self.gattServer.nus_service.tx;
        let mut buffer = [0u8; 32];
        let binary_string = b"pingas";
        buffer[..binary_string.len()].copy_from_slice(binary_string);

        // use indicate for all of these later? not sure how semantics work
        if let Err(e) = tx.notify(self.connection, &buffer).await { 
            warn!("[ble] failed to send ping: {:?}", e);
            Err(TransError::NoAck)//placeholder
        } else {
            info!("[ble] ping sent successfully");
            Ok(())
        }    
    }

    async fn refresh_data(&mut self, widget_id: WidgetId) -> TransResult<ByteArray> {
        // let tx = self.gattServer.nus_service.tx;
        // let mut request = [0u8; CHARACTERISTIC_SIZE];
        //
        // request[0..4].copy_from_slice(&widget_id.0.to_le_bytes()); //le
        //
        // tx.notify(self.connection, &request).await
        //     .map_err(|_| TransError::NoAck)?; //placeholder
        // Ok()
        todo!();
    }
}
