use serde::Deserialize;
use serde::Serialize;

pub type ByteArray = heapless::Vec<u8, 1024>;

#[derive(Serialize, Deserialize)]
pub struct WidgetId(u32);


#[derive(Serialize, Deserialize)]
enum MemError {
    NoAck,
    WidgetNotFound,
}

type MemResult<T> = Result<T, MemError>



#[derive(Serialize, Deserialize)]
enum Javelin {
    // inital setup
    SetWidgets(Widget),

    // get currently flashed widgets
    GetWidget(WidgetId),

    // respond to prods with updated data
    Data((WidgetId, ByteArray)),

    // Battery Level
    GetBatteryLevel,

    /// Set device configuration
    SetConfig(DeviceConfig),
}

#[derive(Serialize, Deserialize)]
enum Needle {
    SendWidget(Widget),
    Prod(WidgetId),

}


// ble {Javelin::SetWidget} -> BLE -> Device -> BLE (ack) -> btleplug -> Ok(())
#[derive(Serialize, Deserialize)]
struct Widget {
    id: u8,
    data: ByteArray,
}

#[derive(Serialize, Deserialize)]
struct DeviceConfig {
    dark_mode: bool,
}
// trait MemTransport {
//     pub fn send_javelin(javelin: Javelin) -> MemResult<Option<Needle>> ;
//     pub fn send_needle(needle: Needle) -> MemResult<Option<Javelin>> ;
// }

// struct MemBLEtransport {

// }

// impl MemTransport for MemBLEtransport {

//     pub fn send_javelin(javelin: Javelin) -> MemResult<Option<Needle>> {
//         todo!()
//     }

//     pub fn send_needle(needle: Needle) -> MemResult<Option<Javelin>> {
//         todo!()
//     }

// }



// struct MemoriTransport<T: MemTransport> {
//     transport: T,
// }
