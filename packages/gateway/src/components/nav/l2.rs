use super::*;

#[inline_props]
pub fn OrgTools(cx: Scope) -> Element {
    cx.render(rsx! {
        h3 { class: "mb-2 text-xs uppercase text-gray-500 font-medium", "Org Admin" }
        ul { class: "mb-8 text-sm font-medium",
            NavItem {
                to: "/hardware",
                name: "Devices",
            }
            NavItem {
                to: "/users",
                name: "Users",
            }
            NavItem {
                to: "/raw",
                name: "Logs",
            }
        }
    })
}
