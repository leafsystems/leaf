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
use nb::block;

use dwm1001::{
    block_timeout,
    dw1000::{
        mac,
        ranging::{self, Message as _RangingMessage},
        RxConfig,
    },
    nrf52832_hal::{
        gpio::{p0::P0_17, Level, Output, PushPull},
        pac::SPIM2,
        rng::Rng,
        Delay, Spim, Timer,
    },
    prelude::*,
    DWM1001,
};

#[entry]
fn main() -> ! {
    defmt::info!("hello!");

    let mut dwm1001 = DWM1001::take().unwrap();

    let mut delay = Delay::new(dwm1001.SYST);
    let mut rng = Rng::new(dwm1001.RNG);

    let mut led = dwm1001.pins.GPIO_12.into_push_pull_output(Level::Low);

    dwm1001.DW_RST.reset_dw1000(&mut delay);
    let mut dw1000 = dwm1001.DW1000.init().expect("Failed to initialize DW1000");

    dw1000
        .enable_tx_interrupts()
        .expect("Failed to enable TX interrupts");
    dw1000
        .enable_rx_interrupts()
        .expect("Failed to enable RX interrupts");

    let mut dw_irq = dwm1001.DW_IRQ;
    let mut gpiote = dwm1001.GPIOTE;

    // These are the hardcoded calibration values from the dwm1001-examples
    // repository[1]. Ideally, the calibration values would be determined using
    // the proper calibration procedure, but hopefully those are good enough for
    // now.
    //
    // [1] https://github.com/Decawave/dwm1001-examples
    dw1000
        .set_antenna_delay(16456, 16300)
        .expect("Failed to set antenna delay");

    // Set network address
    dw1000
        .set_address(
            mac::PanId(0x0d57),                  // hardcoded network id
            mac::ShortAddress(rng.random_u16()), // random device address
        )
        .expect("Failed to set address");

    let mut task_timer = Timer::new(dwm1001.TIMER0);
    let mut timeout_timer = Timer::new(dwm1001.TIMER1);

    task_timer.start(1_000_000u32);

    let mut buf = [0; 128];

    loop {
        // Flash the lights
        led.set_high();
        delay.delay_ms(50u32);
        led.set_low();

        // Clear the timer in case we already received a message
        timer.task_clear();

        // Send a ping
        defmt::info!("Sending message");
        let mut sending = ranging::Ping::new(&mut dw1000)
            .expect("Failed to initiate ping")
            .send(dw1000)
            .expect("Failed to initiate ping transmission");

        let b = sending.wait();

        block!(b).expect("Failed to send data");
        dw1000 = sending.finish_sending().expect("Failed to finish sending");

        // wait for the response
        defmt::info!("Receving started");
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
                defmt::info!("Receiving successful, no error occured");
                message
            }
            Err(e) => {
                defmt::info!("An timeout occured");
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
