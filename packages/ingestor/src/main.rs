use dioxus::prelude::*;
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::stream::StreamExt;
use std::time::Duration;
use tokio_serial::SerialPortBuilderExt;
use uart_types::{DataReading, DATA_BUF_SIZE};

fn main() {
    dioxus::desktop::launch(app)
}

pub fn app(cx: Scope) -> Element {
    let msgs = use_poll_uart(&cx);

    cx.render(rsx!(
        div {
            h1 { "Messages: " }
            table {
                tr {
                    th { "id" }
                    th { "temp" }

                    th { "acce-x" }
                    th { "acce-y" }
                    th { "acce-z" }

                    th { "gyro-x" }
                    th { "gyro-y" }
                    th { "gyro-z" }

                    th { "distance" }
                },
                msgs.read().msgs.iter().rev().take(256).map(|msg| match msg {
                    UartUpdate::Ranging {
                        id,
                        temp,
                        accel: (accel_x, accel_y, accel_z),
                        gyro: (gyro_x, gyro_y, gyro_z),
                        distance,
                    } => rsx! {
                        tr {
                            key: "{distance}",
                            td { "{id}" }
                            td { "{temp}"}

                            td { "{accel_x}" }
                            td { "{accel_y}" }
                            td { "{accel_z}" }

                            td { "{gyro_x}" }
                            td { "{gyro_y}" }
                            td { "{gyro_z}" }

                            td { "{distance}"}
                        }
                    },
                })
            }
        }
    ))
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub enum UartUpdate {
    Ranging {
        id: u8,
        temp: f32,
        accel: (i16, i16, i16),
        gyro: (f32, f32, f32),
        distance: u64,
    },
}

pub enum DeviceCommand {
    Range { target: u8 },
    RangLoop,
    StopRanging,
}

pub struct UartViewState {
    pub msgs: Vec<UartUpdate>,
    pub device: Option<String>,
}

pub fn use_poll_uart(cx: &ScopeState) -> &UseRef<UartViewState> {
    let msgs = use_ref(cx, || UartViewState {
        msgs: Vec::new(),
        device: None,
    });

    use_coroutine(cx, |mut rx: UnboundedReceiver<DeviceCommand>| {
        to_owned![msgs];
        async move {
            println!("starting routine");

            let info = tokio_serial::available_ports()
                .expect("Port discovery failed")
                .into_iter()
                .find(|f| f.port_name.starts_with("tty.usbmodem000760024964"))
                .expect("No connected basestations found");

            let mut port = tokio_serial::new(info.port_name, 115200)
                .parity(tokio_serial::Parity::None)
                .timeout(Duration::from_millis(5000))
                .open_native_async()
                .unwrap();

            use uart_types::{DataReading, DATA_BUF_SIZE};
            // let mut readings = [DataReading::default(); DATA_BUF_SIZE];

            let mut readings = [DataReading::default(); DATA_BUF_SIZE];

            use zerocopy::AsBytes;
            let mut idx = 0;

            loop {
                use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

                println!("reading bytes...");

                let as_bytes = readings.as_bytes_mut();
                let num_read = port.read_exact(as_bytes).await.unwrap();

                println!("reading bytes complete {:?}", num_read);

                // msgs.write().msgs.clear();

                for DataReading {
                    gyro_x,
                    gyro_y,
                    gyro_z,
                    accel_x,
                    accel_y,
                    accel_z,
                } in readings
                {
                    msgs.write().msgs.push(UartUpdate::Ranging {
                        id: 0,
                        temp: 12.0,
                        accel: (accel_x, accel_y, accel_z),
                        gyro: (gyro_x, gyro_y, gyro_z),
                        distance: idx,
                    });
                    idx += 1;
                }
            }

            // while let Some(msg) = rx.next().await {
            //     match msg {
            //         DeviceCommand::Range { .. } => {
            //             use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
            //             port.read_exact(&mut recv_buf).await.unwrap();
            //             let msg = postcard::from_bytes(&recv_buf).unwrap();
            //             msgs.write().msgs.push(msg);
            //         }
            //         DeviceCommand::RangLoop => todo!(),
            //         DeviceCommand::StopRanging => todo!(),
            //     }
            // }
        }
    });

    msgs
}
