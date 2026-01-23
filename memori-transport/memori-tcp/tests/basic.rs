use memori_tcp::{TcpRequest, TcpResponse, TcpTransport};
use std::time::Duration;
use tokio::time::sleep;
use transport::DeviceTransport;
use transport::HostTransport;

#[test]
pub fn test() {
    // Spawn device on its own thread with its own runtime
    let device_thread = std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let device = TcpTransport::new_device(Box::new(device_handler)).await;
            sleep(Duration::from_secs(5)).await;
        });
    });

    // Spawn host on its own thread with its own runtime
    let host_thread = std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut host = TcpTransport::new_host(Box::new(host_handler)).await;
            host.get_battery_level().await.unwrap()
        })
    });

    let batt = host_thread.join().expect("should join fine");
    assert_eq!(batt, 10);

    device_thread.join().expect("device thread should finish");
}

pub fn host_handler(req: TcpRequest) -> TcpResponse {
    match req {
        TcpRequest::GetBatteryLevel => {
            unimplemented!()
        }
    }
}

pub fn device_handler(req: TcpRequest) -> TcpResponse {
    println!("device got request for battery!");
    match req {
        TcpRequest::GetBatteryLevel => TcpResponse::RespondBatteryLevel(10),
    }
}
