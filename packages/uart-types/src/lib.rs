#![no_std]

use serde::{Deserialize, Serialize};
use zerocopy::{AsBytes, FromBytes};

pub const DATA_BUF_SIZE: usize = 512;

#[repr(packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, AsBytes, FromBytes, Serialize, Deserialize)]
pub struct DataReading {
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
}

#[derive(Serialize, Deserialize)]
pub enum UartRequest {
    Range { to: u8 },
    Ping,
    Health,
}

#[derive(Serialize, Deserialize)]
pub enum UartResponse {
    Range { distance_mm: u32 },
    Health,
}
