//! Range measurement anchor node
//!
//! This is an anchor node used for range measurement. Anchors have a known
//! location, and provide the support infrastructure requires by tag nodes to
//! determine their own distance from the available anchors.
//!
//! Currently, distance measurements have a highly inaccurate result. One reason
//! that could account for this is the lack of antenna delay calibration, but
//! it's possible that there are various hidden bugs that contribute to this.

#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;

use dwm1001::{nrf52832_hal::Timer, DWM1001};

#[entry]
fn main() -> ! {
    defmt::info!("Launching tag!");

    let mut dwm: DWM1001 = DWM1001::take().unwrap();

    let mut timer = Timer::new(dwm.TIMER0);

    let out: [u8; 6] = [
        0x70, // P
        0x69, // I
        0x6e, // N
        0x67, // G
        0x0d, // CR
        0x0a, // LF
    ];

    loop {
        dwm.leds.D12.enable();
        tag::delay_timer(&mut timer, 20_000); // 20ms
        dwm.leds.D12.disable();
        tag::delay_timer(&mut timer, 250_000); // 230ms

        dwm.uart.write(&out).unwrap();
    }
}
