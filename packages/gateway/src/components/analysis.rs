use dioxus::prelude::*;

#[inline_props]
pub fn Analysis(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Analysis" }
    })
}
