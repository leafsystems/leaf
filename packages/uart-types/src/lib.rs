#![no_std]

use serde::{Deserialize, Serialize};

#[repr(packed)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    zerocopy::FromBytes,
    zerocopy::AsBytes,
)]
pub struct DataReading {
    pub distance_mm: u64,
    pub timestamp: u64,
    pub anchor: u16,
    pub tag: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum GatewayCommand {
    RegisterAnchor(u32),
    RegisterTag(u32),
    DropAnchor(u32),
}
