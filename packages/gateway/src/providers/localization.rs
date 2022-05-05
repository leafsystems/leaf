use dioxus::prelude::*;
use uart_types::DataReading;

use crate::components::tables::{ListingRow, SiteListing};
use crate::providers::hardware::{self, GatewayInfo};

pub struct LocalizedPosition {
    pub x: f64,
    pub y: f64,
    pub id: u16,
}

pub fn use_localized_data(
    cx: &ScopeState,
) -> (
    &Vec<LocalizedPosition>,
    Option<DataReading>,
    Option<DataReading>,
) {
    let anchor_readings = use_atom_ref(cx, hardware::RAW_READINGS);
    let tag_readings = use_atom_ref(cx, hardware::TAG_READINGS);

    let data = cx.use_hook(|_| vec![]);

    data.clear();

    let mut a1_tag_reading = None;
    let mut a2_tag_reading = None;

    for (tag_id, tag_readings) in tag_readings.read().iter() {
        let mut _anchors = anchor_readings.read();
        let mut anchors = _anchors.iter();
        let a1_reading = anchors.next();
        let a2_reading = anchors.next();

        if let (Some((a1_id, a1)), Some((a2_id, a2))) = (a1_reading, a2_reading) {
            a1_tag_reading = a1.get(tag_id).and_then(|f| f.last()).cloned();
            a2_tag_reading = a2.get(tag_id).and_then(|f| f.last()).cloned();

            if let (Some(a1_reading), Some(a2_reading)) = (a1_tag_reading, a2_tag_reading) {
                let intersections = get_intersections(
                    0.0,
                    0.0,
                    a2_reading.distance_mm as f64,
                    3200.0,
                    0.0,
                    a1_reading.distance_mm as f64,
                );

                if let Some((x3, y3, x4, y4)) = intersections {
                    let x = 100.0 * x4 as f64 / 3200.0;
                    let y = (100.0 * y4 as f64 / 6400.0) - 10.0;

                    log::info!("Successfully localized {:?}", (x, y));

                    data.push(LocalizedPosition { x, y, id: *tag_id });
                }
            }
        }
    }

    (data, a1_tag_reading, a2_tag_reading)
}

pub fn get_intersections(
    x0: f64,
    y0: f64,
    r0: f64,
    x1: f64,
    y1: f64,
    r1: f64,
) -> Option<(f64, f64, f64, f64)> {
    // circle 1: (x0, y0), radius r0
    // circle 2: (x1, y1), radius r1
    let d = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();

    if d > r0 + r1 {
        return None;
    }

    if d < (r0 - r1).abs() {
        return None;
    }

    let a = (r0.powi(2) - r1.powi(2) + d.powi(2)) / (2.0 * d);
    let h = (r0.powi(2) - a.powi(2)).sqrt();
    let x2 = x0 + a * (x1 - x0) / d;
    let y2 = y0 + a * (y1 - y0) / d;
    let x3 = x2 + h * (y1 - y0) / d;
    let y3 = y2 - h * (x1 - x0) / d;

    let x4 = x2 - h * (y1 - y0) / d;
    let y4 = y2 + h * (x1 - x0) / d;

    Some((x3, y3, x4, y4))
}

pub fn use_dummy_data(cx: &ScopeState) -> &Vec<LocalizedPosition> {
    cx.use_hook(|_| {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut out_pts = vec![];

        let center_xs = [37.0, 75.0];
        let center_ys = [25.0, 55.0, 85.0];

        let mut idx = 0;
        for x in center_xs.iter() {
            for y in center_ys.iter() {
                for _ in 0..30 {
                    let fx: f64 = x + rng.gen_range(0.0..10.0) - 10.0;
                    let fy: f64 = y + rng.gen_range(0.0..10.0) - 10.0;

                    out_pts.push(LocalizedPosition {
                        x: fx,
                        y: fy,
                        id: idx,
                    });
                    idx += 1;
                }
            }
        }

        out_pts
    })
}
