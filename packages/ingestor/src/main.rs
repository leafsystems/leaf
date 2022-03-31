#![allow(non_snake_case)]

use dioxus::{desktop::tao::dpi::LogicalSize, prelude::*, router::Router};

use components::*;
mod components {
    pub mod datalog;
    pub mod historical;
    pub mod home;
    pub mod nav;
    pub mod settings;
    pub mod site;
    pub mod sites;
    pub mod uart_controller;
}

mod providers {
    pub mod data;
    pub mod sites;
    pub mod uart;
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        link { href: "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css", rel: "stylesheet" }
        script { src: "https://cdn.plot.ly/plotly-1.52.3.min.js" }
        style { "body {{ height: 100vh; padding: 0; margin: 0; }}" }
        div { class: "mx-auto lg:ml-80",
            Router {
                nav::VerticalNav { }
                section { class: "py-8",
                    div { class: "overflow-hidden container mx-auto px-4",
                        Route { to: "/", "Home" }
                        Route { to: "/sites", sites::Sites {} }
                        Route { to: "/site/:site", site::Dashboard {} }
                        Route { to: "/site/:site/dashboard", site::Dashboard {} }
                        Route { to: "/site/:site/hardware", site::Hardware {} }
                        Route { to: "/site/:site/raw", site::Raw {} }
                        Route { to: "/site/:site/analysis", site::Analysis {} }
                        Route { to: "/site/:site/setup", site::Setup {} }
                        Route { to: "/site/:site/developer", site::Developer {} }
                        Route { to: "", "Err 404 - Not Found" }
                    }
                }
            }
        }
    })
}

fn main() {
    env_logger::init();

    dioxus::desktop::launch_cfg(app, |c| {
        c.with_window(|c| {
            c.with_title("LEAF App")
                .with_inner_size(LogicalSize::new(1600, 1300))
        })
    })
}
