use super::*;

struct AlertContent {
    status: AlertStatus,
    title: String,
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum AlertStatus {
    Info,
    Warning,
    Critical,
}

pub fn Alert(cx: Scope) -> Element {
    let alerts = use_ref(&cx, || {
        vec![
            AlertContent {
                status: AlertStatus::Info,
                title: "Shipment #20912 delievered".to_string(),
            },
            AlertContent {
                status: AlertStatus::Warning,
                title: "High latency on picks in room #2".into(),
            },
            AlertContent {
                status: AlertStatus::Critical,
                title: "Projected delays on line #7".into(),
            },
        ]
    });

    cx.render(rsx! {
        div { class: "p-6 m-3 bg-white shadow rounded overflow-hidden h-60",
            h3 { class: "mb-2 text-xl font-bold flex flex-row justify-between",
                "Active Alerts"
                Link {
                    class: "p-1 my-auto bg-gray-400 hover:bg-indigo-600 rounded text-xs text-white",
                    to: "/alerts",
                    "Configure"
                }
            }
            ul { class: "min-h-40",
                alerts.read().iter().enumerate().map(|(id, alert)| rsx! {
                    li { class: "mt-4", key: "{alert.title}",
                        AlertItem {
                            status: alert.status,
                            title: "{alert.title}",
                            onclear: move |_| {
                                alerts.write().remove(id);
                            },
                        }
                    }
                })
                alerts.read().is_empty().then(|| rsx! {
                    p { class: "my-6 text-gray-500", "No alerts..." }
                })
            }
        }
    })
}

#[inline_props]
fn AlertItem<'a>(
    cx: Scope<'a>,
    status: AlertStatus,
    title: &'a str,
    onclear: EventHandler<'a, ()>,
) -> Element {
    let color = match status {
        AlertStatus::Info => "green",
        AlertStatus::Warning => "yellow",
        AlertStatus::Critical => "red",
    };

    cx.render(rsx! {
        div { class: "p-2 bg-{color}-50 border-l-4 border-{color}-500 rounded-r-lg",
            div { class: "flex items-center",
                span { class: "inline-block mr-2",
                    svg { view_box: "0 0 20 20", width: "20", fill: "none", height: "20", xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M10 0C4.5 0 0 4.5 0 10C0 15.5 4.5 20 10 20C15.5 20 20 15.5 20 10C20 4.5 15.5 0 10 0ZM10 15C9.4 15 9 14.6 9 14C9 13.4 9.4 13 10 13C10.6 13 11 13.4 11 14C11 14.6 10.6 15 10 15ZM11 10C11 10.6 10.6 11 10 11C9.4 11 9 10.6 9 10V6C9 5.4 9.4 5 10 5C10.6 5 11 5.4 11 6V10Z",
                            fill: "{color}"
                        }
                    }
                }
                h3 { class: "text-{color}-800 font-sm", "{title}" }
                button {
                    class: "ml-auto",
                    onclick: move |_| onclear.call(()),
                    svg { class: "text-{color}-800", width: "12", height: "12", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 12 12",
                        path {
                            fill: "currentColor",
                            d: "M6.93341 6.00008L11.1334 1.80008C11.4001 1.53341 11.4001 1.13341 11.1334 0.866748C10.8667 0.600081 10.4667 0.600081 10.2001 0.866748L6.00008 5.06675L1.80008 0.866748C1.53341 0.600081 1.13341 0.600081 0.866748 0.866748C0.600082 1.13341 0.600082 1.53341 0.866748 1.80008L5.06675 6.00008L0.866748 10.2001C0.733415 10.3334 0.666748 10.4667 0.666748 10.6667C0.666748 11.0667 0.933415 11.3334 1.33341 11.3334C1.53341 11.3334 1.66675 11.2667 1.80008 11.1334L6.00008 6.93341L10.2001 11.1334C10.3334 11.2667 10.4667 11.3334 10.6667 11.3334C10.8667 11.3334 11.0001 11.2667 11.1334 11.1334C11.4001 10.8667 11.4001 10.4667 11.1334 10.2001L6.93341 6.00008Z"
                        }
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn Analysis<'a>(
    cx: Scope<'a>,
    title: &'static str,
    description: &'static str,
    children: Element<'a>,
) -> Element {
    cx.render(rsx! {
        div { class: "p-6 m-3 bg-white shadow rounded overflow-hidden",
            h3 { class: "mb-2 text-xl font-bold flex flex-row justify-between",
                "Metric: {title}"
                Link {
                    class: "p-1 my-auto bg-gray-400 hover:bg-indigo-600 rounded text-xs text-white",
                    to: "/analysis",
                    "Configure"
                }
            }
            p { class: "mb-6 text-gray-500", "{description}" }
            children
        }
    })
}
