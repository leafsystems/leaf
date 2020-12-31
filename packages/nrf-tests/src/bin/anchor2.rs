//! periodically send a ping out and then wait for replies.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use dwm1001::{
    block_timeout, debug,
    dw1000::{mac, ranging, RxConfig, TxConfig},
    nrf52832_hal::{gpio::Level, rng::Rng, Delay, Timer},
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

    let mut dwm1001 = DWM1001::take().unwrap();
    let mut rng = Rng::new(dwm1001.RNG);

    let mut led = dwm1001.pins.GPIO_12.into_push_pull_output(Level::Low);
    let mut delay = Delay::new(dwm1001.SYST);
    dwm1001.DW_RST.reset_dw1000(&mut delay);
    let mut dw1000 = dwm1001.DW1000.init().unwrap();

    // dw1000
    //     .enable_tx_interrupts()
    //     .expect("Failed to enable TX interrupts");
    // dw1000
    //     .enable_rx_interrupts()
    //     .expect("Failed to enable RX interrupts");

    let mut timer = Timer::new(dwm1001.TIMER0);

    // Set network address
    dw1000
        .set_address(
            mac::PanId(0x0d57),                  // hardcoded network id
            mac::ShortAddress(rng.random_u16()), // random device address
        )
        .expect("Failed to set address");

    let mut buf = [0; 1024];
    loop {
        // Flash the lights
        led.set_high();
        delay.delay_ms(50u32);
        led.set_low();

        // Clear the timer in case we already received a message
        timer.task_clear();

        // Send a ping
        info!("Sending message");
        let mut sending = ranging::Ping::new(&mut dw1000)
            .expect("Failed to initiate ping")
            .send(dw1000)
            .expect("Failed to initiate ping transmission");
        block!(sending.wait()).expect("Failed to send data");
        dw1000 = sending.finish_sending().expect("Failed to finish sending");

        // wait for the response
        info!("Receving started");
        let mut receiving = dw1000
            .receive(RxConfig {
                frame_filtering: false,
                ..RxConfig::default()
            })
            .expect("Failed to start receiver");

        timer.start(500_000u32);
        let result = block_timeout!(&mut timer, receiving.wait(&mut buf));
        dw1000 = receiving
            .finish_receiving()
            .expect("Failed to finish receiving");

        let message = match result {
            Ok(message) => {
                info!("Receiving successful, no error occured");
                message
            }
            Err(e) => {
                info!("An timeout occured");
                // info!("Error: {:?}\n", error);
                // info!("Error: {:?}\n", error);
                continue;
            }
        };

        let b = b"message received";
        match dwm1001.uart.write(&(b.clone())) {
            Ok(_) => {}
            Err(e) => defmt::info!("writing to uart failed"),
        }
    }
}
