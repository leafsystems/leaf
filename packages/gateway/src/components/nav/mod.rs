pub use dioxus::prelude::*;
pub use horizontal::HorizontalNav;

mod horizontal;
mod icons;
mod l1;
mod l2;
mod l3;
mod user_tools;

pub fn VerticalNav(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "navbar-backdrop fixed lg:hidden inset-0 bg-gray-800 opacity-10" }
        nav {
            class: "fixed top-0 left-0 bottom-0 flex flex-col w-3/4 lg:w-60 sm:max-w-xs pt-6 pb-8 bg-white border-r overflow-y-auto",
            div { class: "flex w-full items-center px-6 pb-6 mb-6 lg:border-b border-blue-50",
                a { class: "text-xl font-semibold", href: "#",
                    img { class: "w-16", alt: "", width: "auto", src: "https://avatars.githubusercontent.com/u/42215498?s=200&v=4" }
                }
            }
            div { class: "px-4 pb-6",
                l3::Tools {}
                l2::OrgTools {}
                l1::Admin {}
                user_tools::UserTools {}
            }
        }
    })
}

#[inline_props]
fn NavItem<'a>(cx: Scope<'a>, name: &'a str, children: Element<'a>, to: &'a str) -> Element {
    let route = use_route(&cx);

    let is_current = route.url().path().starts_with(cx.props.to);

    let active_class = if route.url().path() == cx.props.to {
        "bg-indigo-500 text-white"
    } else {
        "text-gray-500"
    };

    cx.render(rsx! {
        li {
            Link {
                class: "flex items-center pl-3 py-3 pr-4 {active_class} rounded",
                to: "{to}",
                span { class: "inline-block mr-3",
                    icons::icon_0 {}
                }
                span { "{name}" }
                children.is_some().then(|| rsx! {
                    span {
                        class: "inline-block ml-auto hover:bg-gray-500",
                        onclick: move |evt| {
                            evt.cancel_bubble();
                        },
                        icons::icon_8 {}
                    }
                })
            }
            div { class: "px-4",
                is_current.then(|| rsx! { children })
            }
        }
    })
}

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
