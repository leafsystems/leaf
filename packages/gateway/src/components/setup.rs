use dioxus::prelude::*;

#[inline_props]
pub fn Setup(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Setup"
        }
    })
}
