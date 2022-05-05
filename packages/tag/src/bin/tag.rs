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

use lis2dh12::{Lis2dh12, RawAccelerometer};

use defmt_rtt as _;
use panic_probe as _;
use tag::{configure_rx, configure_tx, Msg, UwbPerformance};

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

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::debug!("Launching tag");

    let mut chip = dwm1001::DWM1001::take().unwrap();
    let mut delay = Delay::new(chip.SYST);
    let mut rng = Rng::new(chip.RNG);
    let mut timeout_timer = Timer::new(chip.TIMER0);
    let mut buf = [0u8; 1024];

    let mut radio = tag::build_radio(
        &mut chip.DW_RST,
        chip.DW1000,
        &mut delay,
        rng.random_u16(),
        UwbPerformance::Medium,
    );

    let mut accelerometer = tag::build_accelerometer(
        chip.LIS2DH12,
        lis2dh12::Mode::HighResolution,
        lis2dh12::Odr::Hz10,
    );

    loop {
        defmt::info!("Pinging... for ranging request");

        // Send the ping
        let mut pinging = ranging::Ping::new(&mut radio)
            .unwrap()
            .send(&mut radio)
            .unwrap();

        nb::block!(pinging.wait_transmit()).expect("Failed to send data");

        // Wait for the ranging request
        let mut receiving = radio.receive().expect("Failed to receive message");

        // Set timer for timeout
        timeout_timer.start(1_000_000u32);

        let result = block_timeout!(&mut timeout_timer, receiving.wait(&mut buf));

        if let Ok(Ok(Some(request))) = result
            .as_ref()
            .map(ranging::Request::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>)
        {
            // Return with a ranging response
            let mut sending = ranging::Response::new(&mut radio, &request)
                .expect("Failed to initiate response")
                .send(&mut radio)
                .expect("Failed to initiate response transmission");

            timeout_timer.start(100_000u32);

            nb::block!(sending.wait_transmit()).expect("Failed to send ranging response");
        };

        delay.delay_ms(100u32);
    }
}
