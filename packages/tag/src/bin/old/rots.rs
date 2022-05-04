#![no_main]
#![no_std]

use dwm1001::dw1000::Ready;
use nrf52832_hal::{
    gpio::{p0::P0_17, Output, PushPull},
    pac::SPIM2,
    Spim,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {}
}
