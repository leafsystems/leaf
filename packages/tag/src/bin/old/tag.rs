//! Range measurement basestation
//!
//! This is a tag acting as a base station, collecting distances to mobile tags.
//!
//! The anchor/tag example does the distance calculation *at the tag* which is less useful for applications where
//! the tags are very "dumb".
//!
//! Instead, the basestation intiates the ranging request and records the distance over defmt.

#![no_main]
#![no_std]

use lis2dh12::RawAccelerometer;

use defmt_rtt as _;
use tag::Msg;

use nrf52832_hal::{
    gpio::p0::{self},
    Twim,
};
use panic_probe as _;

use dwm1001::{
    block_timeout,
    dw1000::{
        mac,
        ranging::{self, Message as _RangingMessage},
        RxConfig,
    },
    nrf52832_hal::{
        gpio::{p0::P0_17, Output, PushPull},
        pac::SPIM2,
        rng::Rng,
        Delay, Spim, Temp, Timer,
    },
    prelude::*,
};

static ID: Option<&str> = core::option_env!("BASE_STATION_ID");

#[cortex_m_rt::entry]
fn main() -> ! {
    let _our_id: u8 = ID.unwrap_or("0").parse().unwrap();

    defmt::debug!("Launching basestation");

    let mut chip = dwm1001::DWM1001::take().unwrap();

    let mut delay = Delay::new(chip.SYST);
    let mut rng = Rng::new(chip.RNG);

    chip.DW_RST.reset_dw1000(&mut delay);
    let mut radio = chip
        .DW1000
        .init(&mut delay)
        .expect("Failed to initialize DW1000");

    // radio
    //     .enable_tx_interrupts()
    //     .expect("Failed to enable TX interrupts");
    // radio
    //     .enable_rx_interrupts()
    //     .expect("Failed to enable RX interrupts");

    // These are the hardcoded calibration values from the dwm1001-examples
    // repository[1]. Ideally, the calibration values would be determined using
    // the proper calibration procedure, but hopefully those are good enough for
    // now.
    //
    // [1] https://github.com/Decawave/dwm1001-examples
    radio
        .set_antenna_delay(16456, 16300)
        .expect("Failed to set antenna delay");

    // Set network address
    radio
        .set_address(
            mac::PanId(0x0d57),                  // hardcoded network id
            mac::ShortAddress(rng.random_u16()), // random device address
        )
        .expect("Failed to set address");

    let mut timer = Timer::new(chip.TIMER0);

    let mut buffer1 = [0; 1024];
    let mut buffer2 = [0; 1024];

    let mut temp = Temp::new(chip.TEMP);

    let mut uart_buf = [0u8; 24];

    loop {
        defmt::info!("Sending ping");

        chip.leds.D10.enable();
        delay.delay_ms(10u32);
        chip.leds.D10.disable();

        let mut sending = ranging::Request::new(&mut radio, &ping)
            .expect("Failed to initiate request")
            .send(radio, tag::configure_tx())
            .expect("Failed to initiate request transmission");

        nb::block!(sending.wait_transmit()).expect("Failed to send data");
        radio = sending.finish_sending().expect("Failed to finish sending");

        defmt::debug!("Request sent Transmission sent. Waiting for response.");

        /*
        Decode the ranging request and respond with a ranging response
        */
        let request =
            match ranging::Request::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>(&message) {
                Ok(Some(request)) => request,
                Ok(None) | Err(_) => {
                    defmt::info!("Ignoring message that is not a request\n");
                    continue;
                }
            };

        defmt::info!("Ranging request received. Preparing to send ranging response.");

        chip.leds.D12.enable();
        delay.delay_ms(10u32);
        chip.leds.D12.disable();

        // Wait for a moment, to give the tag a chance to start listening for
        // the reply.
        delay.delay_ms(10u32);

        // Send ranging response
        let mut sending = ranging::Response::new(&mut radio, &request)
            .expect("Failed to initiate response")
            .send(radio, tag::configure_tx())
            .expect("Failed to initiate response transmission");

        nb::block!(sending.wait_transmit()).expect("Failed to send data");
        radio = sending.finish_sending().expect("Failed to finish sending");

        defmt::info!("Ranging response sent");

        chip.leds.D9.enable();
        delay.delay_ms(10u32);
        chip.leds.D9.disable();

        // throttle us to 10hz max
        delay.delay_ms(50u32);
    }
}
