mod socket;
use common_interfaces::{GatewayEvent, HitCount};
use rocket::{get, launch, response::content, routes, State};
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

#[get("/count")]
fn index(hit_count: State<RwLock<HitCount>>) -> content::Html<String> {
    let counts = &hit_count.read().unwrap().events;
    content::Html(serde_json::to_string(counts).unwrap())
}

// #[get("/updates")]
// fn updates<'x>() -> impl Responder<'x> {
//     let tc = TestCounterInner { next: 0 };
//     let tc = BufReader::with_capacity(BUF_SIZE, tc);
//     let ch = rocket::response::Stream::from(tc);
//     let ct = ContentType::parse_flexible("text/event-stream; charset=utf-8").unwrap();
//     Content(ct, ch)
// }

#[get("/ring")]
fn ring(hit_count: State<RwLock<HitCount>>) -> content::Html<String> {
    let evt = GatewayEvent {
        time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        temp: 68,
        orientation_x: 0,
        orientation_y: 0,
        orientation_z: 0,
        gateway_name: "Gateway0".into(),
    };
    let out = format!("success! {:#?}", &evt);
    hit_count.write().unwrap().events.push(evt);
    content::Html(format!("{:?}", out))
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, ring])
        .manage(RwLock::new(HitCount::default()))
        .attach(rocket_cors::CorsOptions::default().to_cors().unwrap())
}
