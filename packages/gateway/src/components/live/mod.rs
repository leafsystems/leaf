use dioxus::prelude::*;

use crate::components::tables::{ListingRow, SiteListing};
use crate::providers::hardware::{self, GatewayInfo};
use crate::providers::localization::*;

#[inline_props]
pub fn Live(cx: Scope) -> Element {
    let (data, r1, r2) = use_localized_data(&cx);

    let banner = match (r1, r2) {
        (None, None) => rsx! { "No readings" },
        (None, Some(r2)) => rsx! { "r2: {r2.distance_mm}" },
        (Some(r1), None) => rsx! { "r1: {r1.distance_mm}" },
        (Some(r1), Some(r2)) => rsx! {
           "r2: {r2.distance_mm}"
           "r1: {r1.distance_mm}"
        },
    };

    cx.render(rsx! {
        div {
            div { class: "flex flex-row",
                LivePosition {
                    data: data
                }
                h1 { banner }
            }
            AnchorList {}
            TagList {}
        }
    })
}

fn AnchorList(cx: Scope) -> Element {
    let gw1 = match use_read(&cx, hardware::CONNECTED_GATEWAY_1) {
        Some(GatewayInfo { port_name }) => rsx! {
            ListingRow {
                dark: false,
                disp_id: "#GW-1",
                img: "",
                name: "Gateway 1 - Inventory",
                sub_title: "{port_name}",
                date: "4/27/2022",
                status: "Connected",
                archetype: "Zone 1",
            }
        },
        None => rsx! {
            ListingRow {
                dark: false,
                disp_id: "#GW-1",
                img: "",
                name: "Gateway 1 - Inventory",
                sub_title: "pending...",
                date: "4/27/2022",
                status: "Disconnected",
                archetype: "Zone 1",
            }
        },
    };

    let gw2 = match use_read(&cx, hardware::CONNECTED_GATEWAY_2) {
        Some(GatewayInfo { port_name }) => rsx! {
            ListingRow {
                dark: true,
                disp_id: "#GW-2",
                img: "",
                name: "Gateway 1 - Inventory",
                sub_title: "{port_name}",
                date: "4/27/2022",
                status: "Connected",
                archetype: "Zone 1",
            }
        },
        None => rsx! {
            ListingRow {
                dark: true,
                disp_id: "#GW-2",
                img: "",
                name: "Gateway 1 - Inventory",
                sub_title: "pending...",
                date: "4/27/2022",
                status: "Disconnected",
                archetype: "Zone 1",
            }
        },
    };

    cx.render(rsx! {
        SiteListing {
            title: "Anchors",
            gw1,
            gw2
        }
    })
}

pub fn TagList(cx: Scope) -> Element {
    let readings = use_atom_ref(&cx, hardware::TAG_READINGS);

    cx.render(rsx! {
        SiteListing {
            title: "Tags",
            readings.read().keys().map(|tag_id| {
                cx.render(rsx! {
                    ListingRow {
                        key: "{tag_id}",
                        dark: false,
                        disp_id: "#TAG-{tag_id}",
                        img: "",
                        name: "Mobile Tag - {tag_id}",
                        sub_title: "Connected",
                        date: "4/27/2022",
                        status: "Connected",
                        archetype: "Zone 1",
                    }
                })
            })
        }
    })
}

#[inline_props]
pub fn LivePosition<'a>(cx: Scope<'a>, data: &'a [LocalizedPosition]) -> Element {
    cx.render(rsx! {
        div { class: "p-6 m-3 bg-white shadow rounded overflow-hidden mx-auto",
            div { class: "flex flex-wrap items-stretch w-full",
                h1 { class: "w-full p-4 text-xl font-bold rounded-t", "Live Position Data" }
            }
            div { class: "container relative",
                svg { style: "width: 900px; height: 550px; top: 0; left: 0; position: relative; z-index: 5;",
                    data.iter().map(|pos| rsx! {
                        circle {
                            key: "{pos.id}",
                            cx: "{pos.x}%",
                            cy: "{pos.y}%",
                            r: "5",
                            fill: "red",
                            "{pos.id}",
                        }
                    })
                }
                FactorySvg {}
            }
        }
    })
}

fn FactorySvg(cx: Scope) -> Element {
    let contents = format!(
        r###"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="{width}" height="{height}" viewBox="-0.5 -0.5 582 401" content="&lt;mxfile host=&quot;app.diagrams.net&quot; modified=&quot;2022-04-27T07:05:21.389Z&quot; agent=&quot;5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Safari/605.1.15&quot; etag=&quot;CR92XFMqd2D_Ij51B_bz&quot; version=&quot;17.4.6&quot; type=&quot;google&quot;&gt;&lt;diagram id=&quot;YUASAVdgl9w67dldLP__&quot; name=&quot;Page-1&quot;&gt;3ZfLcpswFIafhiUZXbguG9tJN105M+kuoxgBmgBiZDngPH0FCAMF0mamlDhsLP060pG+X5LBwJu0vBckj3/wgCYGAkFp4K2BEPR8T/1UylkryNFKJFigtU7YszeqRaDVEwvocRAoOU8ky4figWcZPciBRoTgxTAs5Mkwa04iOhL2B5KM1UcWyLhRPRt0+nfKorjNDIFuSUkbrIVjTAJe9CS8M/BGcC6bUlpuaFLRa7k0/e5mWi8TEzSTf9PB8UP2UpAzwvfuEyQ/efT2aLaYX0ly0ivWs5XnFkEk+CnXYVRIWk6BJ89tOBhPDF6WqzYK5SmV4qxC9ECmewOBbSEX264DFEBETWg3o+htg6Db1IvOA9vTmeIef6vFT7Tv0SVdh0YVNJ0PkHL/DEpxygJaDQIMfFvETNJ9Tg5Va6GOh9JimaqkW6iKIc+k3u5qcfh2FnAf5DsezuJ9h+56NOGI5kO9gxCAV4LVATeg/8DBhrXwiDACE4S9pQCjWcD4SwCGDliZMJ4lbH8JwurGWJmwNUv4Wu5e3P4bfpZrwZ5Fal0p0vUvAmeWqXOlTNc/+t6I6S6TogL1T4kKLolkPFN106/6hyxJNjzhoh4cO4D4dWDVsaeH9aP0oxT8hfZa0Fa9ZYElzbLc3w7AhFlw+m1uIbP8sVklk4s5NWVU4AGgOHzAqG82ANaiRpn/0ShV7b4k67beBzne/QI=&lt;/diagram&gt;&lt;/mxfile&gt;"><defs/><g><rect x="0" y="0" width="580" height="400" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="none"/><rect x="60" y="43" width="200" height="80" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="none"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 198px; height: 1px; padding-top: 83px; margin-left: 61px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 17px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: none; white-space: normal; word-wrap: normal;">Table 1</div></div></div></foreignObject><text x="160" y="88" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="17px" text-anchor="middle">Table 1</text></switch></g><rect x="60" y="160" width="200" height="80" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="none"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 198px; height: 1px; padding-top: 200px; margin-left: 61px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 17px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: none; white-space: normal; word-wrap: normal;">Table 3</div></div></div></foreignObject><text x="160" y="205" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="17px" text-anchor="middle">Table 3</text></switch></g><rect x="60" y="273" width="200" height="80" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="none"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 198px; height: 1px; padding-top: 313px; margin-left: 61px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 17px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: none; white-space: normal; word-wrap: normal;">Table 5</div></div></div></foreignObject><text x="160" y="318" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="17px" text-anchor="middle">Table 5</text></switch></g><rect x="320" y="43" width="200" height="80" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="none"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 198px; height: 1px; padding-top: 83px; margin-left: 321px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 17px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: none; white-space: normal; word-wrap: normal;">Table 2</div></div></div></foreignObject><text x="420" y="88" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="17px" text-anchor="middle">Table 2</text></switch></g><rect x="320" y="160" width="200" height="80" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="none"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 198px; height: 1px; padding-top: 200px; margin-left: 321px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 17px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: none; white-space: normal; word-wrap: normal;">Table 4</div></div></div></foreignObject><text x="420" y="205" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="17px" text-anchor="middle">Table 4</text></switch></g><rect x="320" y="273" width="200" height="80" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="none"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 198px; height: 1px; padding-top: 313px; margin-left: 321px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 17px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: none; white-space: normal; word-wrap: normal;">Table 6</div></div></div></foreignObject><text x="420" y="318" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="17px" text-anchor="middle">Table 6</text></switch></g><rect x="470" y="173" width="180" height="40" fill="#60a917" stroke="#2d7600" transform="rotate(-90,560,193)" pointer-events="none"/><g transform="translate(-0.5 -0.5)rotate(-90 560 193)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 178px; height: 1px; padding-top: 193px; margin-left: 471px;"><div data-drawio-colors="color: #ffffff; " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 17px; font-family: Helvetica; color: rgb(255, 255, 255); line-height: 1.2; pointer-events: none; white-space: normal; word-wrap: normal;">Entrace</div></div></div></foreignObject><text x="560" y="198" fill="#ffffff" font-family="Helvetica" font-size="17px" text-anchor="middle">Entrace</text></switch></g><rect x="-70" y="173" width="180" height="40" fill="#d80073" stroke="#a50040" transform="rotate(90,20,193)" pointer-events="none"/><g transform="translate(-0.5 -0.5)rotate(90 20 193)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 178px; height: 1px; padding-top: 193px; margin-left: -69px;"><div data-drawio-colors="color: #ffffff; " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 17px; font-family: Helvetica; color: rgb(255, 255, 255); line-height: 1.2; pointer-events: none; white-space: normal; word-wrap: normal;">Exit</div></div></div></foreignObject><text x="20" y="198" fill="#ffffff" font-family="Helvetica" font-size="17px" text-anchor="middle">Exit</text></switch></g></g><switch><g requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility"/><a transform="translate(0,-5)" xlink:href="https://www.diagrams.net/doc/faq/svg-export-text-problems" target="_blank"><text text-anchor="middle" font-size="10px" x="50%" y="100%">Text is not SVG - cannot display</text></a></switch></svg>"###,
        width = "900px",
        height = "550px",
    );

    cx.render(rsx! {
        div { dangerous_inner_html: "{contents}", position: "absolute", top: "0", left: "0", z_index: "1" }
    })
}

// let localized = {
//     let mut _readings = readings.borrow();
//     let mut list = _readings.iter();

//     match (
//         list.next()
//             .and_then(|(_, r)| r.last().map(|f| f.distance_mm)),
//         list.next()
//             .and_then(|(_, r)| r.last().map(|f| f.distance_mm)),
//     ) {
//         (Some(lr), Some(lx)) => Some({
//             // perform *real* localization
//             (lr, lx)
//         }),
//         _ => None,
//     }
// }
// .map(|(x, y)| format!("x: {x}, y: {y}"))
// .unwrap_or_else(|| "Not available".to_string());
