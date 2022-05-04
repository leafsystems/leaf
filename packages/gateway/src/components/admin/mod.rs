use dioxus::prelude::*;

#[inline_props]
pub fn Admin(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Admin" }
    })
}
