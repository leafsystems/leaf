pub use dioxus::prelude::*;

#[inline_props]
pub fn NavButton<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full block py-5 px-3 text-center text-gray-700 border border-gray-300 rounded-md hover:bg-gray-100 hover:text-gray-900",
            width: "25vw",
            height: "12vh",
            display: "fixed",
            children
        }
    })
}
