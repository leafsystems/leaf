use dioxus::core::to_owned;
use dioxus::prelude::*;
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, sync::RwLock, time::Duration};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;
use uart_types::{DataReading, DATA_BUF_SIZE};
use zerocopy::AsBytes;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
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
    pub secs_left_poll: u64,
}

impl UartViewState {
    pub fn save_data(&self) {
        let id = uuid::Uuid::new_v4();
        std::fs::write(
            format!("data/raw/{id}.json"),
            serde_json::to_string_pretty(&self.msgs).unwrap(),
        )
        .expect("couldn't write launch db");
    }
}

pub fn use_poll_uart(cx: &ScopeState) -> &UseRef<UartViewState> {
    let msgs = use_ref(cx, || UartViewState {
        msgs: Vec::new(),
        device: None,
        secs_left_poll: 0,
    });

    use_coroutine(cx, |mut rx: UnboundedReceiver<DeviceCommand>| {
        to_owned![msgs];
        async move {
            println!("starting routine");

            let mut info = tokio_serial::available_ports()
                .expect("Port discovery failed")
                .into_iter()
                .map(|f| {
                    println!("found port: {:?}", f);
                    f
                })
                .find(|f| f.port_name == "/dev/tty.usbmodem0007600249831");

            loop {
                if info.is_some() {
                    break;
                }

                msgs.write().secs_left_poll = 3;
                for _ in 0..3 {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    msgs.write().secs_left_poll -= 1;
                }

                info = tokio_serial::available_ports()
                    .expect("Port discovery failed")
                    .into_iter()
                    .find(|f| f.port_name.starts_with("tty.usbmodem000760024964"));

                if info.is_some() {
                    break;
                }
            }

            let info = info.unwrap();

            msgs.write().device = Some(info.port_name.clone());

            let mut port = tokio_serial::new(info.port_name, 115200)
                .parity(tokio_serial::Parity::None)
                .timeout(Duration::from_millis(5000))
                .open_native_async()
                .unwrap();

            port.write(&[
                0x70, // P
                0x69, // I
                0x6e, // N
                0x67, // G
                0x0d, // CR
                0x0a, // LF
            ])
            .await
            .unwrap();

            let mut readings = [DataReading::default(); DATA_BUF_SIZE];

            let mut idx = 0;

            loop {
                println!("reading bytes...");

                for item in readings.iter_mut() {
                    let as_bytes = item.as_bytes_mut();

                    // let as_bytes = readings.as_bytes_mut();
                    let num_read = port.read_exact(as_bytes).await.unwrap();
                    println!("bytes read: {}", num_read);
                }

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
