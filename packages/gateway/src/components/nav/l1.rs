use super::*;

#[inline_props]
pub fn Admin(cx: Scope) -> Element {
    cx.render(rsx! {
        h3 { class: "mb-2 text-xs uppercase text-gray-500 font-medium", "Leaf Admin" }
        ul { class: "mb-8 text-sm font-medium",
            NavItem {
                to: "/admin",
                name: "Admin",
            }
        }
    })
}
