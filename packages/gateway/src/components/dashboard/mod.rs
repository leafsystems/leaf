use crate::components::tables::ListingRow;
use crate::icons;
use crate::providers::hardware;
use crate::providers::localization::*;
use dioxus::prelude::*;

pub mod cards;
pub mod plots;

/*

The dashboard includes
- current position of all tags (including hover) and ability to select zone
- list of active alerts / logs per site
- Quick analysis -> derived data from the site data

And then a list of sites, their statuses, and a quick way to filter for them
*/
pub fn Dashboard(cx: Scope) -> Element {
    let data = use_dummy_data(&cx);

    cx.render(rsx! {
        div {
            DashboardTopNav {}
            div { class: "flex flex-wrap items-stretch",
                div { class: "w-full lg:w-1/3 p-4",
                    cards::Alert {}
                    cards::Analysis {
                        title: "Average Pick Time",
                        description: "Average time to pick an item from inventory",
                        plots::LinePlot {
                            data: &[1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0],
                            divid: "pick-time",
                        }
                    }
                }
                div { class: "w-full lg:w-2/3 my-auto",
                    crate::components::live::LivePosition {
                        data: data
                    }
                }
            }
            crate::components::tables::SiteListing {
                title: "Anchors",
                (0..3).map(|f| rsx! {
                    ListingRow {
                        key: "{f}",
                        dark: f % 2 == 0,
                        disp_id: "#GW-{f}",
                        img: "",
                        name: "Gateway {f} - Inventory",
                        sub_title: "connected...",
                        date: "4/27/2022",
                        status: "Disconnected",
                        archetype: "Zone 1",
                    }
                })
            }
            crate::components::tables::SiteListing {
                title: "Tags",
                (0..10).map(|f| rsx! {
                    ListingRow {
                        key: "{f}",
                        dark: f % 2 == 0,
                        disp_id: "#TAG-{f}",
                        img: "",
                        name: "Mobile Tag - {f}",
                        sub_title: "Connected",
                        date: "4/27/2022",
                        status: "Connected",
                        archetype: "Zone 1",
                    }
                })
            }
        }
    })
}

#[inline_props]
pub fn Gateway(cx: Scope, id: u8) -> Element {
    let gateway = use_read(
        &cx,
        match id {
            0 => hardware::CONNECTED_GATEWAY_1,
            _ => hardware::CONNECTED_GATEWAY_2,
        },
    );
    let readings = use_atom_ref(&cx, hardware::RAW_READINGS);

    cx.render(rsx! {
        div { class: "px-4" }
    })
}

#[inline_props]
pub fn DashboardTopNav(cx: Scope) -> Element {
    cx.render(rsx! {
        section { class: "py-8 px-6",
            div { class: "flex flex-wrap items-center",
                div { class: "mb-5 lg:mb-0",
                    h2 { class: "mb-2 text-2xl font-bold", "Dashboard" }
                    div { class: "flex items-center",
                        a { class: "flex items-center text-sm text-gray-500", href: "#",
                            span { class: "inline-block mr-2",
                                icons::IconHome {}
                            }
                            span { "Sites" }
                        }
                        span { class: "inline-block mx-4",
                            icons::IconChevron2 {}
                        }
                        a { class: "flex items-center text-sm", href: "#",
                            span { class: "inline-block mr-2",
                                icons::IconChartBars {}
                            }
                            span {
                                select {
                                    option { value: "1", "Primary Site" }
                                    option { value: "1", "Site A" }
                                    option { value: "1", "Site B" }
                                    option { value: "1", "Site C" }
                                    option { value: "1", "Site D" }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

fn DataRangeSelector(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "w-full lg:w-auto lg:ml-auto mb-5 lg:mb-0",
            div { class: "flex items-center lg:justify-end",
                label { class: "mr-3 text-sm text-gray-500", r#for: "", "From" }
                div { class: "flex p-2 pl-4 pr-2 bg-white border rounded",
                    span { class: "inline-block mr-2",
                        icons::IconCalendar {}
                    }
                    select { class: "w-full pr-2 text-xs text-gray-500", id: "", name: "",
                        option { value: "1", "20/04/2021" }
                        option { value: "1", "20/04/2021" }
                        option { value: "1", "20/04/2021" }
                        option { value: "1", "20/04/2021" }
                    }
                }
                label { class: "mx-3 text-sm text-gray-500", r#for: "", "to" }
                div { class: "flex mr-3 p-2 pl-4 pr-2 bg-white border rounded",
                    span { class: "inline-block mr-2",
                        icons::IconCalendar {}
                    }
                    select { class: "w-full pr-2 text-xs text-gray-500", id: "", name: "",
                        option { value: "1", "20/04/2021" }
                        option { value: "1", "20/04/2021" }
                        option { value: "1", "20/04/2021" }
                        option { value: "1", "20/04/2021" }
                    }
                }
            }
        }
    })
}
