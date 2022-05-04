use dioxus::prelude::*;

#[inline_props]
pub fn Logs(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Setup" }
    })
}
