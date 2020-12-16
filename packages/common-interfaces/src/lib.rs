use instant;
use serde::{Deserialize, Serialize};
use serde_millis;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GatewayEvent {
    pub time: u128,
    pub temp: i32,
    pub orientation_x: i32,
    pub orientation_y: i32,
    pub orientation_z: i32,
    pub gateway_name: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HitCount {
    pub events: Vec<GatewayEvent>,
}
