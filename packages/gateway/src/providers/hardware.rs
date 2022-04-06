use std::{num, time::Duration};

use dioxus::{
    core::to_owned,
    fermi::{use_atom_root, Readable},
    prelude::*,
};
use futures::StreamExt;
use serde::__private::de;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;
use uart_types::{DataReading, GatewayCommand};
use zerocopy::AsBytes;

pub enum Command {
    AddTag,
    AddAnchor,
}

pub static CONNECTED_GATEWAY: Atom<Option<GatewayInfo>> = |_| None;
pub static READINGS: AtomRef<Vec<DataReading>> = |_| vec![];

pub struct GatewayInfo {
    pub name: String,
}

pub fn use_hardware_service(cx: &ScopeState) {
    let root = use_atom_root(cx).clone();
    let (readings, readings_id) = cx.use_hook(|_| {
        let val = root.register(READINGS, cx.scope_id());
        (val, READINGS.unique_id())
    });

    use_coroutine(cx, |mut rx: UnboundedReceiver<Command>| {
        to_owned![readings, readings_id];
        async move {
            'main: loop {
                let info = tokio_serial::available_ports()
                    .unwrap()
                    .into_iter()
                    .find(|f| f.port_name.contains("usbmodem00076"));

                let info = match info {
                    Some(info) => info,
                    None => {
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        root.set(CONNECTED_GATEWAY.unique_id(), None as Option<GatewayInfo>);
                        continue;
                    }
                };

                let mut port = tokio_serial::new(info.port_name.clone(), 115200)
                    .parity(tokio_serial::Parity::None)
                    .timeout(Duration::from_millis(5000))
                    .open_native_async()
                    .unwrap();

                root.set(
                    CONNECTED_GATEWAY.unique_id(),
                    Some(GatewayInfo {
                        name: info.port_name,
                    }),
                );

                let mut reading = DataReading::default();

                loop {
                    // while let Some(msg) = rx.next().await {
                    //     match msg {
                    //         Command::AddTag => {
                    //             postcard::to_slice(&GatewayCommand::RegisterTag(10), &mut cmd_buf)
                    //                 .unwrap();
                    //             port.write_all(&cmd_buf).await.unwrap();
                    //         }

                    //         Command::AddAnchor => {
                    //             postcard::to_slice(&GatewayCommand::RegisterAnchor(10), &mut cmd_buf)
                    //                 .unwrap();
                    //             port.write_all(&cmd_buf).await.unwrap();
                    //         }
                    //     }
                    // }

                    match port.read_exact(reading.as_bytes_mut()).await {
                        Err(_) => continue,
                        Ok(_) => {
                            log::debug!("fetched reading {:?}", reading);

                            readings.borrow_mut().push(reading);
                            root.force_update(readings_id);
                        }
                    }
                }
            }
        }
    });
}

pub struct Anchor {
    // meters
    pos_x: f32,
    pos_y: f32,
    history: Vec<RawReading>,
}

pub struct RawReading {
    // millis
    timestamp: u64,

    // meters
    distance: f32,
}
