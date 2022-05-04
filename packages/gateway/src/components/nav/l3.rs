use super::*;

#[inline_props]
pub fn Tools(cx: Scope) -> Element {
    cx.render(rsx! {
        h3 { class: "mb-2 text-xs uppercase text-gray-500 font-medium", "Tools" }
        ul { class: "mb-8 text-sm font-medium",
            NavItem {
                to: "/",
                name: "Dashboard",
            }
            NavItem {
                to: "/analysis",
                name: "Analysis",
            }
            NavItem {
                to: "/alerts",
                name: "Alerts",
            }
            NavItem {
                to: "/live",
                name: "Live",
            }
        }
    })
}
