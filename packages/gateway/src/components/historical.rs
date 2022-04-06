use dioxus::prelude::*;

use crate::providers::data::{RunVertical, RUN_DATA};
use crate::providers::uart::UartUpdate;

struct RunCard {
    id: &'static str,
    contents: &'static str,
}

pub fn HistoricalList(cx: Scope) -> Element {
    let runs = use_read(&cx, RUN_DATA);

    cx.render(rsx! {
        div {
            h1 { "Historical Data" }
            ul {
                runs.runs.keys().map(|f| rsx!{
                    HistoricalCard { key: "{f}", id: *f }
                })
            }
        }
    })
}

#[inline_props]
pub fn HistoricalCard(cx: Scope, id: uuid::Uuid) -> Element {
    cx.render(rsx! {
        li {
            Link {
                to: "/games/{id}",
                "{id}",
            }
        }
    })
}

#[inline_props]
pub fn HistoricalItem(cx: Scope) -> Element {
    let id = uuid::Uuid::parse_str(use_route(&cx).last_segment().unwrap()).unwrap();

    let runs = use_read(&cx, RUN_DATA);
    let run = runs.runs.get(&id).unwrap();

    cx.render(rsx! {
        div {
            h1 { "Historical Data" }
            h2 { "Viewing run: {id}" }
            Link {
                to: "/games",
                "Back"
            }
            GyroPlot { vals: run }
            AccelPlot { vals: run }
        }
    })
}

#[inline_props]
fn GyroPlot<'a>(cx: &'a ScopeState, vals: &'a RunVertical) -> Element<'a> {
    use plotly::{common::Mode, Plot, Scatter};
    use plotly::{common::Title, Layout};

    let mut plot = Plot::new();

    plot.add_trace(
        Scatter::new(0..vals.gyro_x.len(), vals.gyro_x.iter().copied())
            .mode(Mode::Markers)
            .legend_group("Gyro X"),
    );

    plot.add_trace(
        Scatter::new(0..vals.gyro_y.len(), vals.gyro_y.iter().copied())
            .mode(Mode::Markers)
            .legend_group("Gyro Y"),
    );

    plot.add_trace(
        Scatter::new(0..vals.gyro_z.len(), vals.gyro_z.iter().copied())
            .mode(Mode::Markers)
            .legend_group("Gyro Z"),
    );

    plot.set_layout(Layout::new().title(Title::new("Gyroscope")));

    build_plotly(&cx, plot, "gyro")
}

#[inline_props]
fn AccelPlot<'a>(cx: &'a ScopeState, vals: &'a RunVertical) -> Element<'a> {
    use plotly::{common::Mode, Plot, Scatter};
    use plotly::{common::Title, Layout};

    let mut plot = Plot::new();

    plot.add_trace(
        Scatter::new(0..vals.accel_x.len(), vals.accel_x.iter().copied())
            .mode(Mode::Markers)
            .legend_group("Accel X"),
    );

    plot.add_trace(
        Scatter::new(0..vals.accel_y.len(), vals.accel_y.iter().copied())
            .mode(Mode::Markers)
            .legend_group("Accel Y"),
    );

    plot.add_trace(
        Scatter::new(0..vals.accel_z.len(), vals.accel_z.iter().copied())
            .mode(Mode::Markers)
            .legend_group("Accel Z"),
    );

    plot.set_layout(Layout::new().title(Title::new("Accel")));

    build_plotly(&cx, plot, "accel")
}

fn build_plotly<'a>(cx: &'a ScopeState, plot: plotly::Plot, id: &str) -> Element<'a> {
    // need to trim out the div
    let raw = plot.to_inline_html(Some(id));
    let raw_wo_div = &raw[128..raw.len() - 9];

    cx.render(rsx! {
        div {
            div { id: "{id}", class: "" }
            script { "{raw_wo_div}" }
        }
    })
}
