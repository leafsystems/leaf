#![no_std]
#![no_main]

use cortex_m_rt::entry;
use dwm1001::{
    debug,
    dw1000::{mac, TxConfig},
    nrf52832_hal::{gpio::Level, Delay},
    prelude::*,
    print, DWM1001,
};
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use nb::block;
use nrf_tests as _;

#[entry]
fn main() -> ! {
    debug::init();

    let dwm1001 = DWM1001::take().unwrap();
    let mut dw1000 = dwm1001.DW1000.init().unwrap();
    let mut led = dwm1001.pins.GPIO_12.into_push_pull_output(Level::Low);
    let mut delay = Delay::new(dwm1001.SYST);

    loop {
        let mut sending = dw1000
            .send(
                b"ping",
                mac::Address::broadcast(&mac::AddressMode::Short),
                None,
                TxConfig::default(),
            )
            .expect("Failed to start receiver");

        block!(sending.wait()).expect("Failed to send data");

        dw1000 = sending.finish_sending().expect("Failed to finish sending");

        led.set_high();
        delay.delay_ms(50u32);
        led.set_low();
        delay.delay_ms(50u32);
        // print!(".");
        defmt::info!("sent!");
    }
}
