#![no_std]
#![no_main]

use nb::block;

use dwm1001::{
    debug,
    dw1000::{mac, TxConfig},
    print, DWM1001,
};

// use nrf52832_hal as hal;
// use cortex_m::asm;
use cortex_m_rt::entry;
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    debug::init();

    let dwm1001 = DWM1001::take().unwrap();
    let mut dw1000 = dwm1001.DW1000.init().unwrap();

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

        // print!(".");
        defmt::info!("sent!");
    }
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    defmt::error!("panicked");
    exit()
}

pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

// use nrf52840_hal as hal;
// use nrf52840_hal as hal;
// use nrf52832_hal as hal;
// use nrf52832_hal::gpio::Level;
