use super::*;

use crate::icons;

use super::VerticalNav;

pub fn HorizontalNav(cx: Scope) -> Element {
    let site_name = "Olin Demo";
    let abbrev = "OD";

    cx.render(rsx! {
        section { class: "py-5 px-6 bg-white shadow",
            nav { class: "relative",
                div { class: "flex items-center",
                    a {
                        class: "group relative dropdown px-4 text-purple-500 hover:text-purple-700 cursor-pointer font-bold text-base uppercase tracking-wide z-10",
                        icons::IconHome {}
                        div { class: "group-hover:block dropdown-menu absolute hidden h-auto",
                            VerticalNav {}
                        }
                    }
                    div { class: "flex items-center mr-auto",
                        button { class: "flex items-center",
                            span {
                                class: "flex justify-center items-center mr-3 w-10 h-10 bg-indigo-500 text-sm text-white rounded-full",
                                "{abbrev}"
                            }
                            p { class: "text-sm font-medium mr-2", "{site_name}" }
                            span { class: "inline-block -mb-px",
                                icons::ChevronUpDown {}
                            }
                        }
                    }
                    div { class: "ml-auto lg:hidden",
                        button { class: "flex items-center",
                            icons::Empty {}
                        }
                    }
                    QuickIcons {}
                    UserCard {}
                }
            }
        }
    })
}

fn UserCard(cx: Scope) -> Element {
    let full_name = "Jon Kelley";

    cx.render(rsx!(
        div { class: "hidden lg:block",
            button { class: "flex items-center",
                img { class: "w-10 h-10 mr-2 rounded-full object-cover object-right",
                    alt: "",
                    src: "https://images.unsplash.com/photo-1568602471122-7832951cc4c5?ixlib=rb-1.2.1&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=1050&amp;q=80",
                }
                p { class: "text-sm mr-3", "{full_name}" }
                span {  icons::ChevronUpDown {} }
            }
        }
    ))
}

fn QuickIcons(cx: Scope) -> Element {
    cx.render(rsx!(
        ul { class: "hidden lg:flex items-center space-x-6 mr-6",
            li {
                a { class: "text-gray-200 hover:text-gray-300",
                    href: "#",
                    icons::SearchGlass {}
                }
            }
            li {
                a { class: "text-gray-200 hover:text-gray-300",
                    href: "#",
                    icons::ChatMessage {}
                }
            }
            li {
                a { class: "text-gray-200 hover:text-gray-300",
                    href: "#",
                    icons::Bell {}
                }
            }
        }
    ))
}
