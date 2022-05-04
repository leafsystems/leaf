#![allow(non_snake_case)]
use components::*;
use dioxus::prelude::*;
use warp::ws::Ws;
use warp::Filter;

pub mod components {
    pub(crate) mod admin;
    pub(crate) mod alerts;
    pub(crate) mod analysis;
    pub(crate) mod dashboard;
    pub(crate) mod developer;
    pub(crate) mod hardware;
    pub(crate) mod live;
    pub(crate) mod nav;
    pub(crate) mod raw;
    pub(crate) mod setup;
    pub(crate) mod tables;
}

mod providers {
    pub mod data;
    pub mod hardware;
    pub mod localization;
    pub mod sites;
}

mod icons;

fn app(cx: Scope) -> Element {
    providers::hardware::use_hardware_service(&cx);

    // div { class: "mx-auto lg:ml-60",
    // nav::VerticalNav {}
    // section { class: "overflow-hidden container mr-auto ml-2",
    cx.render(rsx! {
        Router {
            div { class: "mx-auto",
                nav::HorizontalNav {}
                section { class: "overflow-hidden container mx-auto",
                    Route {
                        to: "/",
                        dashboard::Dashboard {}
                    }
                    Route {
                        to: "/analysis",
                        analysis::Analysis {}
                    }
                    Route {
                        to: "/live",
                        live::Live {}
                    }
                    Route {
                        to: "/alerts",
                        alerts::Alerts {}
                    }
                    Route {
                        to: "/hardware",
                        hardware::Hardware {}
                    }
                    Route {
                        to: "/setup",
                        setup::Setup {}
                    }
                    Route {
                        to: "/logs",
                        raw::Logs {}
                    }
                    Route {
                        to: "/developer",
                        developer::Developer {}
                    }
                    Route {
                        to: "",
                        "Err 404 - Not Found"
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
    let body = view.body(
        r####"
        <script src="https://cdn.tailwindcss.com"></script>
        <script src="https://cdn.plot.ly/plotly-1.52.3.min.js"> </script>
        <style> body { height: 100vh; padding: 0; margin: 0; } </style>
        <script>
        tailwind.config = {
            theme: {
                extend: {
                    colors: {
                    clifford: '#da373d',
                    }
                }
            },
            variants: {
                display:['group-hover']
            }
        }
        </script>

        <script>
        // On page load or when changing themes, best to add inline in `head` to avoid FOUC
        if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
            document.documentElement.classList.add('dark')
        } else {
            document.documentElement.classList.remove('dark')
        }
        </script>

        "####,
    );

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
