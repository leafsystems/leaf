use dioxus::prelude::*;

#[inline_props]
pub fn Developer(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Developer" }
    })
}
