use dioxus::prelude::*;

#[inline_props]
pub fn Alerts(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            Header {}
        }
    })
}

fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        section { class: "pt-8 px-6",
            h2 { class: "mb-4 text-2xl font-bold", "Alerts" }
            div { class: "flex flex-wrap text-sm text-center",
                Link {
                    class: "inline-block w-full md:w-1/2 lg:w-auto mb-4 lg:mb-0 px-4 pb-2 border-b-2 border-indigo-500 text-indigo-500",
                    to: "/alerts",
                    "Log"
                }
                Link {
                    class: "inline-block w-full md:w-1/2 lg:w-auto mb-4 lg:mb-0 px-4 pb-2 text-gray-300 border-b-2 border-transparent hover:border-gray-300",
                    to: "/alerts?filter=warning",
                    "Manage Warnings"
                }
                Link {
                    class: "inline-block w-full md:w-1/2 lg:w-auto mb-4 lg:mb-0 px-4 pb-2 text-gray-300 border-b-2 border-transparent hover:border-gray-300",
                    to: "/alerts?filter=warning",
                    "Scripting"
                }
                Link {
                    class: "inline-block w-full md:w-1/2 lg:w-auto px-4 pb-2 text-gray-300 border-b-2 border-transparent hover:border-gray-300",
                    to: "/alerts?filter=warning",
                    "Returns"
                }
            }
        }
    })
}
