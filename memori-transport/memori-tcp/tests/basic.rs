use memori_tcp::DeviceRequest;
use memori_tcp::DeviceResponse;
use memori_tcp::DeviceTcpTransport;
use memori_tcp::HostRequest;
use memori_tcp::HostResponse;
use memori_tcp::HostTcpTransport;
use std::time::Duration;
use tokio::time::sleep;
use transport::HostTransport;

#[test]
pub fn test() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();

    // Spawn device on its own thread with its own runtime
    let device_thread = std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let device = DeviceTcpTransport::new(device_handler);

            device.connect().await.unwrap();

            sleep(Duration::from_secs(5)).await;
        });
    });

    // Spawn host on its own thread with its own runtime
    let host_thread = std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let host = HostTcpTransport::new(host_handler);
            let mut host = host.connect().await.unwrap();
            host.get_battery_level()
                .await
                .expect("should not have transport error")
        })
    });

    let batt = host_thread.join().expect("should join fine");
    assert_eq!(batt, 10);

    device_thread.join().expect("device thread should finish");
}

pub async fn host_handler(req: DeviceRequest) -> HostResponse {
    match req {
        DeviceRequest::RefreshData(widget_id) => todo!(),
        DeviceRequest::Ping => HostResponse::Pong,
    }
}

pub async fn device_handler(req: HostRequest) -> DeviceResponse {
    match req {
        HostRequest::GetBatteryLevel => DeviceResponse::BatteryLevel(10),
        _ => {
            todo!()
        }
    }
}
