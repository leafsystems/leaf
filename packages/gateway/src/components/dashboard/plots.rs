use super::*;

use plotly::layout::Margin;
use plotly::{common::Mode, Plot, Scatter};
use plotly::{common::Title, Layout};

#[derive(Props)]
pub struct PlotsProps<'a> {
    data: &'a [f64],
    divid: &'static str,
}

/// Generate a revenue vs review plot
pub fn LinePlot<'a>(cx: Scope<'a, PlotsProps<'a>>) -> Element {
    let mut plot = Plot::new();

    plot.add_trace(
        Scatter::<usize, f64>::new(0..cx.props.data.len(), cx.props.data.iter().copied())
            .mode(Mode::LinesMarkersText),
    );

    plot.set_layout(
        Layout::new()
            // .title(Title::new("Distribution of Revenue"))
            .margin(Margin::new().left(20).bottom(20).right(20).top(20)),
    );

    build_plotly(&cx, plot, cx.props.divid)
}

fn build_plotly<'a>(cx: &'a ScopeState, plot: plotly::Plot, id: &str) -> Element<'a> {
    let raw = plot.to_inline_html(Some(id));

    // the id of the plot varies with the length of the id
    // just anchor to the start of the code
    let start = raw.find("window.PLOTLYENV=window.PLOTLYENV").unwrap();
    let raw_wo_div = &raw[start..raw.len() - 9];

    cx.render(rsx! {
        div {
            div { id: "{id}", style: "height: 270px;" }
            script { "{raw_wo_div}" }
        }
    })
}
