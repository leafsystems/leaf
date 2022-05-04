use dioxus::prelude::*;

#[inline_props]
pub fn Hardware(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Hardware" }
    })
}

#[inline_props]
pub fn Raw(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Raw" }
    })
}

#[inline_props]
pub fn Analysis(cx: Scope) -> Element {
    // div {
    //     "Analysis"
    // }
    cx.render(rsx! {
        crate::components::historical::HistoricalList {}
    })
}

#[inline_props]
pub fn Setup(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Setup" }
    })
}

#[inline_props]
pub fn Developer(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Developer" }
    })
}
