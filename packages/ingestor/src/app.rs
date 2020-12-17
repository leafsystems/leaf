use async_std::{prelude::*, sync::RwLock};
use common_interfaces::GatewayEvent;
pub use log::info;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use state::ServerState;
use tide::Request;
use tide_websockets::{Message, WebSocket, WebSocketConnection};

type ServerRequest = Request<Arc<RwLock<ServerState>>>;

pub async fn app() -> Result<(), std::io::Error> {
    let mut app = tide::with_state(ServerState::new());
    app.at("/updates").get(WebSocket::new(socket_handler));
    app.at("/ring").get(ring);
    app.at("/count").get(count);

    let addr = "0.0.0.0:9001";
    // let addr = "127.0.0.1:9001";
    log::info!("Congrats! Server is up and running at http://{}", addr);
    app.listen(addr).await?;

    Ok(())
}

async fn ring(req: ServerRequest) -> tide::Result<String> {
    let evt = GatewayEvent {
        time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        temp: 68,
        orientation_x: 0,
        orientation_y: 0,
        orientation_z: 0,
        gateway_name: "Gateway0".into(),
    };

    info!("New event recoreded at {:#?} ", evt.time);

    let response_msg = serde_json::to_string(&evt)?;

    let mut state = req.state().try_write().unwrap();
    state.events.push(evt.clone());
    match state.sender.send(evt).await {
        Ok(_) => {}
        Err(e) => log::warn!("failed to update listeners {:#?}", e),
    };

    Ok(response_msg)
}

async fn count(req: ServerRequest) -> tide::Result<String> {
    info!("Ring revent recorded");
    let state = req.state();
    let hits = state.as_ref().read().await;
    Ok(serde_json::to_string(&hits.events)?)
}

async fn socket_handler(request: ServerRequest, stream: WebSocketConnection) -> tide::Result<()> {
    // clone the receiver channel
    // subscribe to any updates
    let receiver = request.state().read().await.receiver.clone();
    while let Ok(evt) = receiver.recv().await {
        let response_msg = serde_json::to_string(&evt)?;
        stream.send_string(response_msg).await?;
    }

    Ok(())
}

mod state {
    use super::*;

    pub struct ServerState {
        pub(crate) events: Vec<GatewayEvent>,
        pub(crate) sender: async_std::channel::Sender<GatewayEvent>,
        pub(crate) receiver: async_std::channel::Receiver<GatewayEvent>,
    }

    impl ServerState {
        pub fn new() -> Arc<RwLock<Self>> {
            let (tx, rx) = async_std::channel::bounded(100);

            let state = Self {
                events: Vec::new(),
                sender: tx,
                receiver: rx,
            };

            Arc::new(RwLock::new(state))
        }
    }
}
