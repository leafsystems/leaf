use std::{collections::HashMap, num, time::Duration};

use dioxus::{
    core::to_owned,
    fermi::{use_atom_root, use_atom_state, Readable},
    prelude::*,
};
use futures::StreamExt;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;
use uart_types::{DataReading, GatewayCommand};
use zerocopy::AsBytes;

pub enum Command {
    AddTag,
    AddAnchor,
}

pub static CONNECTED_GATEWAY_1: Atom<Option<GatewayInfo>> = |_| None;
pub static CONNECTED_GATEWAY_2: Atom<Option<GatewayInfo>> = |_| None;

type AnchorId = u16;
type TagId = u16;

pub static RAW_READINGS: AtomRef<HashMap<AnchorId, HashMap<TagId, Vec<DataReading>>>> =
    |_| HashMap::default();

pub static TAG_READINGS: AtomRef<HashMap<u16, Vec<DataReading>>> = |_| HashMap::default();

pub struct GatewayInfo {
    pub port_name: String,
}

pub fn use_hardware_service(cx: &ScopeState) {
    use_poll_gateway(cx, CONNECTED_GATEWAY_1, "000760162072");
    use_poll_gateway(cx, CONNECTED_GATEWAY_2, "000760162068");
}

fn use_poll_gateway(cx: &ScopeState, gateway: Atom<Option<GatewayInfo>>, port_id: &'static str) {
    let root = use_atom_root(cx).clone();

    let (readings, readings_id) = cx.use_hook(|_| {
        let val = root.register(RAW_READINGS, cx.scope_id());
        (val, RAW_READINGS.unique_id())
    });

    let (tag_readings, tag_readings_id) = cx.use_hook(|_| {
        let val = root.register(TAG_READINGS, cx.scope_id());
        (val, TAG_READINGS.unique_id())
    });

    use_coroutine(cx, |_: UnboundedReceiver<()>| {
        to_owned![readings, readings_id, tag_readings, tag_readings_id, root];
        async move {
            loop {
                let info = tokio_serial::available_ports()
                    .unwrap()
                    .into_iter()
                    .find(|f| f.port_name.contains(port_id));

                let info = match info {
                    Some(info) => info,
                    None => {
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        log::debug!("Waiting for gateway {}", port_id);
                        root.set(gateway.unique_id(), None as Option<GatewayInfo>);
                        continue;
                    }
                };

                let mut port = tokio_serial::new(info.port_name.clone(), 115200)
                    .parity(tokio_serial::Parity::None)
                    .timeout(Duration::from_millis(5000))
                    .open_native_async()
                    .unwrap();

                log::debug!("Connected to gateway {}", info.port_name);

                root.set(
                    gateway.unique_id(),
                    Some(GatewayInfo {
                        port_name: info.port_name,
                    }),
                );

                root.force_update(gateway.unique_id());

                let mut reading = DataReading::default();

                while port.read_exact(reading.as_bytes_mut()).await.is_ok() {
                    log::debug!("Reading: {:?}", reading);

                    readings
                        .borrow_mut()
                        .entry(reading.anchor)
                        .or_default()
                        .entry(reading.tag)
                        .or_default()
                        .push(reading);
                    root.force_update(readings_id);

                    tag_readings
                        .borrow_mut()
                        .entry(reading.tag)
                        .or_default()
                        .push(reading);
                    root.force_update(tag_readings_id);
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
