// #![deny(warnings)]
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use dioxus::prelude::*;
use futures_util::{pin_mut, SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::task::LocalPoolHandle;
use warp::ws::{Message, WebSocket};
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let pool = tokio_util::task::LocalPoolHandle::new(16);

    let routes = warp::path::end()
        .map(move || warp::reply::html(content()))
        .or(warp::path("chat")
            .and(warp::ws())
            .and(warp::any().map(move || pool.clone()))
            .map(|ws: warp::ws::Ws, pool| {
                ws.on_upgrade(move |socket| user_connected(socket, pool))
            }));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn app(cx: Scope) -> Element {
    let (dog, set_dog) = use_state(&cx, || None);

    cx.use_hook(|_| {
        to_owned![set_dog];
        cx.spawn(async move {
            #[derive(serde::Deserialize, Debug)]
            struct DogApi {
                message: String,
            }

            loop {
                let resp = reqwest::get("https://dog.ceo/api/breeds/image/random")
                    .await
                    .unwrap()
                    .json::<DogApi>()
                    .await
                    .unwrap();

                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                set_dog(Some(resp.message));
            }
        })
    });

    cx.render(rsx! {
        div { "hello world" },
        dog.as_ref().and_then(|f| cx.render(rsx!{
            img {
                src: "{f}",
                height: "300px",
            }
        }))
    })
}

mod events;

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

async fn user_connected(ws: WebSocket, pool: LocalPoolHandle) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    let (event_tx, event_rx) = mpsc::unbounded_channel();
    let (edits_tx, edits_rx) = mpsc::unbounded_channel();

    let mut edits_rx = UnboundedReceiverStream::new(edits_rx);
    let mut event_rx = UnboundedReceiverStream::new(event_rx);

    let vdom_fut = pool.spawn_pinned(move || async move {
        let mut vdom = VirtualDom::new(app);

        let edits = vdom.rebuild();

        let serialized = serde_json::to_string(&edits.edits).unwrap();
        edits_tx.send(serialized).unwrap();

        loop {
            use futures_util::future::{select, Either};

            let new_event = {
                let vdom_fut = vdom.wait_for_work();

                pin_mut!(vdom_fut);

                match select(event_rx.next(), vdom_fut).await {
                    Either::Left((l, _)) => l,
                    Either::Right((_, _)) => None,
                }
            };

            if let Some(new_event) = new_event {
                vdom.handle_message(dioxus::core::SchedulerMsg::Event(new_event));
            } else {
                let mutations = vdom.work_with_deadline(|| false);
                for mutation in mutations {
                    let edits = serde_json::to_string(&mutation.edits).unwrap();
                    edits_tx.send(edits).unwrap();
                }
            }
        }
    });

    loop {
        use futures_util::future::{select, Either};

        match select(user_ws_rx.next(), edits_rx.next()).await {
            Either::Left((l, _)) => {
                if let Some(Ok(msg)) = l {
                    if let Ok(Some(msg)) = msg.to_str().map(events::parse_ipc_message) {
                        let user_event = events::trigger_from_serialized(msg.params);
                        event_tx.send(user_event).unwrap();
                    }
                }
            }
            Either::Right((edits, _)) => {
                if let Some(edits) = edits {
                    // send the edits to the client
                    if user_ws_tx.send(Message::text(edits)).await.is_err() {
                        break;
                    }
                }
            }
        }
    }

    vdom_fut.abort();
}

fn content() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
  <head>
    <title>Dioxus app</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  </head>
  <body>
    <div id="main"></div>
    <script>
      {interpreter}
      main();
    </script>
  </body>
</html>"#,
        interpreter = include_str!("../src/interpreter.js")
    )
}
