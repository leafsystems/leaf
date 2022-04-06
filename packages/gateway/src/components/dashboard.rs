use dioxus::prelude::*;

use crate::providers::hardware;

#[inline_props]
pub fn Dashboard(cx: Scope) -> Element {
    let gateway = use_read(&cx, hardware::CONNECTED_GATEWAY);
    let _svc = use_coroutine_handle::<hardware::Command>(&cx)?;
    let readings = use_atom_ref(&cx, hardware::READINGS);

    cx.render(rsx! {
        div {
            h1 { class: "text-3xl", "Dashboard" }
            match gateway {
                Some(gateway) => rsx! {
                    div {
                        h1 { class: "text-xl",
                            "Gateway connected: "
                            span { class: "text-gray-500", "{gateway.name}" }
                        }
                        div { class: "flex flex-col",
                            ul {
                                readings.read().iter().rev().map(|reading| rsx!{
                                    li { key: "{reading.timestamp}",
                                        span { class: "text-gray-700", "[{reading.tag}@{reading.timestamp}]: " }
                                        span { class: "text-gray-500", "{reading.distance_mm} mm" }
                                    }
                                })
                            }
                        }
                    }
                },
                None => rsx! {
                    div {
                        "No anchors connected to this gateway!"
                        img {
                            src: "https://media.giphy.com/media/7SF5scGB2AFrgsXP63/giphy.gif"
                        }
                    }
                }
            }
        }
    })
}
