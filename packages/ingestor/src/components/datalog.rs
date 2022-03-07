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

use crate::providers::{
    data::{load_run, RUN_DATA},
    uart::{use_poll_uart, UartUpdate, UartViewState},
};

pub fn Datalog(cx: Scope) -> Element {
    let uart = use_poll_uart(&cx);

    if uart.read().device.is_none() {
        let time_left = uart.read().secs_left_poll;
        return cx.render(rsx! {
            div { "No devices connected." }
            div { "Reconnnecting in {time_left} seconds..." }
        });
    }

    cx.render(rsx!(
        div {
            h1 { "Data" }

            // match **show_raw {
            //     false => rsx!( GraphView { uart: uart.clone() } ),
            // true => rsx!( RawView { uart: uart.clone() } ),
            // }


            RawView { uart: uart.clone() },
        }
    ))
}

#[inline_props]
pub fn GraphView(cx: Scope, uart: UseRef<UartViewState>) -> Element {
    let uart = use_poll_uart(&cx);

    cx.render(rsx! {
        div {
            h1 { "Run Data: "}

            button {
                onclick: move |_| uart.read().save_data(),
                "Save Data to disk!"
            }
        }
    })
}

#[inline_props]
pub fn RawView(cx: Scope, uart: UseRef<UartViewState>) -> Element {
    let show_download = !uart.read().msgs.is_empty();

    let data = use_read(&cx, RUN_DATA);

    let set_Data = use_set(&cx, RUN_DATA);

    cx.render(rsx! {
        if show_download {
            cx.render(rsx!{
                button {
                    onclick: move |_| {
                        uart.read().save_data();
                        set_Data({
                            let mut new = data.clone();
                            new.runs.insert(uuid::Uuid::new_v4(), load_run(uart.read().msgs.clone()));
                            new
                        });
                    },
                    "Save Data!"
                }
            })
        } else {
            None
        }

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
            uart.read().msgs.iter().rev().take(256).map(|msg| match msg {
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
    })
}
