#![allow(non_snake_case)]

use dioxus::desktop::use_window;
use dioxus::router::*;
use dioxus::{desktop::tao::dpi::LogicalSize, prelude::*, router::Router};

mod components {
    pub mod datalog;
    pub mod historical;
    pub mod nav;
    pub mod settings;
    pub mod uart_controller;
}

mod providers {
    pub mod data;
    pub mod uart;
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        link { href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css", rel: "stylesheet" }
        script { src: "https://cdn.plot.ly/plotly-1.52.3.min.js" }
        style { "{BODYSTYLE}" }

        div { class: "bg-gray-200", height: "100vh",
            Router {
                div { class: "overflow-hidden", // its up to each frame to handle its own scrolling
                    Route { to: "/", "Home" }
                    Route { to: "/games", components::historical::HistoricalList {} }
                    Route { to: "/games/:id", components::historical::HistoricalItem {} }
                    Route { to: "/play", components::datalog::Datalog {} }
                    Route { to: "/settings", "Settings" }
                }
                ul { class: "flex flex-wrap fixed bottom-0 inset-x-0 flex",
                    Link { to: "/", components::nav::NavButton { "Home" }}
                    Link { to: "/games", components::nav::NavButton { "Games" }}
                    Link { to: "/play", components::nav::NavButton { "Play" }}
                    Link { to: "/settings", components::nav::NavButton { "Settings" }}
                }
            }
        }
    })
}

const BODYSTYLE: &str = r#"
body {
  height: 100vh;
  padding: 0;
  margin: 0;
}
"#;

fn main() {
    env_logger::init();

    dioxus::desktop::launch_cfg(app, |c| {
        c.with_window(|c| {
            c.with_title("Spinsense Client")
                .with_inner_size(LogicalSize::new(400, 700))
            // .with_resizable(false)
        })
    })
}
