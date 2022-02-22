use std::sync::Arc;

use dioxus::prelude::*;
use futures::lock::Mutex;
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::stream::StreamExt;

use crate::{Msg, UartUpdate};

type SharedReceiver<T> = Arc<Mutex<UnboundedReceiver<T>>>;
pub struct AppProps {
    pub receiver: SharedReceiver<UartUpdate>,
}

pub fn app(cx: Scope<AppProps>) -> Element {
    let msgs = use_poll_uart(&cx, &cx.props.receiver);

    cx.render(rsx!(
        div {
            h1 { "Messages: " }
            table {
                tr {
                    th { "id" }
                    th { "temp" }
                    th { "accel" }
                    th { "distance" }
                },
                msgs.read().iter().rev().take(20).map(|msg| match msg {
                    UartUpdate::Ranging(Msg {
                        id,
                        temp,
                        accel,
                        distance,
                    }) => rsx! {
                        tr {
                            td { "{id}" }
                            td { "{temp}"}
                            td { "{accel:?}" }
                            td { "{distance}"}
                        }
                    },
                })
            }
        }
    ))
}

fn use_poll_uart<'a>(
    cx: &'a ScopeState,
    receiver: &SharedReceiver<UartUpdate>,
) -> &'a UseRef<Vec<UartUpdate>> {
    let msgs = use_ref(cx, Vec::new);

    cx.use_hook(|_| {
        to_owned![msgs, receiver];
        cx.spawn(async move {
            let mut recv = receiver.lock().await;
            while let Some(msg) = recv.next().await {
                msgs.write().push(msg);
            }
        });
    });

    msgs
}
