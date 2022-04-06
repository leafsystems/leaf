use dioxus::prelude::*;

#[inline_props]
pub fn Hardware(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hardware"
        }
    })
}
