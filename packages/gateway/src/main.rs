#![allow(non_snake_case)]
use dioxus::prelude::*;
use warp::ws::Ws;
use warp::Filter;

use components::*;
mod components {
    pub(crate) mod analysis;
    pub(crate) mod dashboard;
    // pub(crate) mod datalog;
    pub(crate) mod developer;
    pub(crate) mod hardware;
    // pub(crate) mod historical;
    pub(crate) mod home;
    pub(crate) mod nav;
    pub(crate) mod raw;
    pub(crate) mod setup;
}

mod providers {
    pub mod data;
    pub mod hardware;
    pub mod sites;
}

fn app(cx: Scope) -> Element {
    providers::hardware::use_hardware_service(&cx);

    cx.render(rsx! {
        Router {
            link { href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css", rel: "stylesheet" }
            script { src: "https://cdn.plot.ly/plotly-1.52.3.min.js" }
            style { "body {{ height: 100vh; padding: 0; margin: 0; }}" }
            div { class: "mx-auto lg:ml-80",
                nav::VerticalNav { }
                section { class: "py-8",
                    div { class: "overflow-hidden container mx-auto px-4",
                        Route { to: "/", dashboard::Dashboard {} }
                        Route { to: "/hardware", hardware::Hardware {} }
                        Route { to: "/setup", setup::Setup {} }
                        Route { to: "/raw", raw::Setup {} }
                        Route { to: "/analysis", analysis::Setup {} }
                        Route { to: "/developer", developer::Setup {} }
                        Route { to: "", "Err 404 - Not Found" }
                    }
                }
            }
        }
    })
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();

    let addr = ([127, 0, 0, 1], 3030);

    let view = dioxus_liveview::new(addr);
    let body = view.body();

    let routes = warp::path::end()
        .map(move || warp::reply::html(body.clone()))
        .or(warp::path("app")
            .and(warp::ws())
            .and(warp::any().map(move || view.clone()))
            .map(|ws: Ws, view: dioxus_liveview::Liveview| {
                ws.on_upgrade(|socket| async move {
                    view.upgrade(socket, app).await;
                })
            }));

    warp::serve(routes).run(addr).await;
}
