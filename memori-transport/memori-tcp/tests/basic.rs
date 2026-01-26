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
    let expected_battery = 10;

    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();

    // Spawn device on its own thread with its own runtime
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let device = DeviceTcpTransport::default();

            let (conn, (mut host_req_rx, mut dev_resp_tx)) = device.connect().await.unwrap();

            tokio::spawn(async move {
                while let Some(req) = host_req_rx.recv().await {
                    let resp = match req {
                        HostRequest::GetBatteryLevel => {
                            DeviceResponse::BatteryLevel(expected_battery)
                        }
                        HostRequest::Ping => DeviceResponse::Pong,
                        HostRequest::SetWidgets(wid) => todo!(),
                        _ => todo!(),
                    };

                    dev_resp_tx.send(resp).unwrap();
                }
            });

            sleep(Duration::from_secs(1)).await;
        });
    });

    // Spawn host on its own thread with its own runtime
    let host_thread = std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let host = HostTcpTransport::default();
            let (mut conn, (dev_req_rx, host_resp_tx)) = host.connect().await.unwrap();

            conn.get_battery_level()
                .await
                .expect("should not have transport error")
        })
    });

    let batt = host_thread.join().expect("should join fine");
    assert_eq!(batt, expected_battery);
}
