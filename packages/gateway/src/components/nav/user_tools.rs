use super::*;

#[inline_props]
pub fn UserTools(cx: Scope) -> Element {
    let show_user_menu = use_state(&cx, || false);
    cx.render(rsx! {
        div { class: "pt-8",
            button {
                class: "flex items-center pl-3 py-3 pr-2 text-gray-500 hover:bg-indigo-50 rounded",
                onclick: move |evt| {
                    show_user_menu.set(!show_user_menu.get());
                    evt.cancel_bubble();
                },
                span { class: "inline-block mr-4",
                    icons::icon_14 {}
                }
                span { "Settings" }
            }
            show_user_menu.then(|| rsx! {
                a { class: "flex items-center pl-3 py-3 pr-2 text-gray-500 hover:bg-indigo-50 rounded",
                    href: "#",
                    span { class: "inline-block mr-4",
                        icons::icon_16 {}
                    }
                    span {
                        "Log Out"
                    }
                }
            })
        }
    })
}
