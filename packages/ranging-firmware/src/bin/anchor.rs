#![no_std]
#![no_main]

// use dwm1001::nrf52832_hal::{prelude::*, timer, Timer};
// use dwm1001::{dw1000::DW1000, nrf52832_hal as hal};
// use dwm1001::{nrf52832_hal::gpio::Level, DWM1001};

use defmt::{debug, info, warn};
use defmt_rtt as _;
use dw1000::{
    mac,
    ranging::{self, Message as _},
    RxConfig, TxConfig,
};
// global logger
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use hal::prelude::*;
use hal::{gpio::Level, Delay, Timer};
use mobile_firmware::{block_timeout, DWM1001};
use nrf52832_hal as hal;
use panic_probe as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut dwm = DWM1001::take().unwrap();
    let mut delay = Delay::new(dwm.SYST);
    dwm.DW_RST.reset_dw1000(&mut delay);
    let mut dw = dwm.DW1000.init().unwrap();
    let mut led = dwm.pins.GPIO_12.into_push_pull_output(Level::Low);
    let mut timer = Timer::new(dwm.TIMER0);

    loop {
        defmt::info!("Sending!");
        let mut sending = dw
            .send(
                b"ping",
                mac::Address::broadcast(&mac::AddressMode::Short),
                None,
                TxConfig::default(),
            )
            .expect("Failed to start receiver");

        block!(sending.wait()).expect("Failed to send data");

        dw = sending.finish_sending().expect("Failed to finish sending");

        led.set_high();
        // delay(&mut timer, 500_000); // 20ms

        // dwm1001.leds.D9.enable();
        delay.delay_ms(100u32);

        led.set_low();
        delay.delay_ms(100u32);
        // print!(".");
    }
}

use nb::block;
fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
    T: hal::timer::Instance,
{
    timer.start(cycles);
    block!(timer.wait()).unwrap();
}
