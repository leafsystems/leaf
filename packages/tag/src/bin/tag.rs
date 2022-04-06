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
use tag::{configure_rx, configure_tx, Msg};

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

    let mut radio = {
        chip.DW_RST.reset_dw1000(&mut delay);

        let mut radio = chip
            .DW1000
            .init(&mut delay)
            .expect("Failed to initialize DW1000");

        radio
            .enable_tx_interrupts()
            .expect("Failed to enable TX interrupts");

        radio
            .enable_rx_interrupts()
            .expect("Failed to enable RX interrupts");

        // Set network address
        radio
            .set_address(
                mac::PanId(0x0d57),                  // hardcoded network id
                mac::ShortAddress(rng.random_u16()), // random device address
            )
            .expect("Failed to set address");

        radio
            .set_antenna_delay(16456, 16300)
            .expect("Failed to set antenna delay");

        radio
    };

    let mut accelerometer = {
        let mut accelerometer =
            Lis2dh12::new(chip.LIS2DH12, lis2dh12::SlaveAddr::Alternative(true))
                .expect("lis2dh12 new failed");

        accelerometer
            .set_mode(lis2dh12::Mode::HighResolution)
            .expect("lis2dh12 set_mode failed");

        accelerometer
            .set_odr(lis2dh12::Odr::Hz10)
            .expect("lis2dh12 set_odr failed");

        accelerometer
            .enable_axis((true, true, true))
            .expect("lis2dh12 enable_axis failed");

        accelerometer
    };

    let mut timeout_timer = Timer::new(chip.TIMER0);
    let mut dw_irq = chip.DW_IRQ;
    let mut gpiote = chip.GPIOTE;
    let mut buf = [0u8; 1024];

    let mut moving = false;

    loop {
        let reading = accelerometer.accel_raw().unwrap();
        let total_movement = 0u64
            .saturating_add(reading.x.abs() as u64)
            .saturating_add(reading.y.abs() as u64)
            .saturating_add(reading.z.abs() as u64);

        if total_movement < 20_000 {
            // tag isn't moving. wait 250ms before retrying
            delay.delay_ms(250u32);
            continue;
        }

        for _ in 0..5 {
            // Send the ping
            let mut pinging = ranging::Ping::new(&mut radio)
                .unwrap()
                .send(radio, configure_tx())
                .unwrap();

            nb::block!(pinging.wait_transmit()).expect("Failed to send data");
            radio = pinging.finish_sending().expect("Failed to finish sending");

            // Wait for the ranging request
            let mut receiving = radio
                .receive(configure_rx())
                .expect("Failed to receive message");

            // Set timer for timeout
            timeout_timer.start(1_000_000u32);

            let result = block_timeout!(&mut timeout_timer, receiving.wait_receive(&mut buf));

            radio = receiving
                .finish_receiving()
                .expect("Failed to finish receiving");

            let request = match result
                .as_ref()
                .map(ranging::Request::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>)
            {
                Ok(Ok(Some(request))) => request,
                e => continue,
            };

            // Return with a ranging response
            let mut sending = ranging::Response::new(&mut radio, &request)
                .expect("Failed to initiate response")
                .send(radio, tag::configure_tx())
                .expect("Failed to initiate response transmission");

            timeout_timer.start(100_000u32);

            nb::block!(sending.wait_transmit()).expect("Failed to send ranging response");

            radio = sending.finish_sending().expect("Failed to finish sending");

            delay.delay_ms(100u32);
        }
    }
}
