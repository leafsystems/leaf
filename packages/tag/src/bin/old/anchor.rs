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
        prelude::*,
        rng::Rng,
        Delay, Spim, Timer,
    },
    prelude::*,
    DWM1001,
};
use tag::{configure_rx, UwbPerformance};

#[entry]
fn main() -> ! {
    defmt::info!("Starting anchor code!");

    let mut chip = DWM1001::take().unwrap();

    let mut delay = Delay::new(chip.SYST);
    let mut rng = Rng::new(chip.RNG);

    chip.DW_RST.reset_dw1000(&mut delay);
    let mut radio = chip
        .DW1000
        .init(&mut delay)
        .expect("Failed to initialize radio");

    radio
        .enable_tx_interrupts()
        .expect("Failed to enable TX interrupts");

    radio
        .enable_rx_interrupts()
        .expect("Failed to enable RX interrupts");

    let mut dw_irq = chip.DW_IRQ;
    let mut gpiote = chip.GPIOTE;

    // These are the hardcoded calibration values from the chip-examples
    // repository[1]. Ideally, the calibration values would be determined using
    // the proper calibration procedure, but hopefully those are good enough for
    // now.
    //
    // [1] https://github.com/Decawave/chip-examples
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

    let mut buf = [0; 128];

    loop {
        let mut receiving = radio
            .receive(tag::configure_rx(UwbPerformance::Medium))
            .expect("Failed to receive message");

        // wait for a message
        defmt::info!("Waiting for receiver interrupt");

        // task_timer.start(500_000_u32);
        // dw_irq.wait_for_interrupts(&mut gpiote, &mut task_timer);

        // finish receiver
        let result = nb::block!(receiving.wait_receive(&mut buf));

        radio = receiving
            .finish_receiving()
            .expect("Failed to finish receiving");

        let message = match result {
            Ok(message) => message,
            _ => {
                defmt::info!("No message received");
                continue;
            }
        };

        chip.leds.D11.enable();
        delay.delay_ms(10u32);
        chip.leds.D11.disable();

        let request = ranging::Request::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>(&message);

        let request = match request {
            Ok(Some(request)) => {
                defmt::info!("Request has been made");
                request
            }
            Ok(None) | Err(_) => {
                defmt::info!("Ignoring message that is not a request\n");
                continue;
            }
        };

        chip.leds.D12.enable();
        delay.delay_ms(10u32);
        chip.leds.D12.disable();

        // Wait for a moment, to give the tag a chance to start listening for
        // the reply.
        delay.delay_ms(10u32);

        // Send ranging response
        let mut sending = ranging::Response::new(&mut radio, &request)
            .expect("Failed to initiate response")
            .send(radio, tag::configure_tx(UwbPerformance::Medium))
            .expect("Failed to initiate response transmission");

        timer.start(100_000u32);

        nb::block!({
            dw_irq.wait_for_interrupts(&mut gpiote, &mut timer);
            sending.wait_transmit()
        })
        .expect("Failed to send ranging response");

        radio = sending.finish_sending().expect("Failed to finish sending");

        chip.leds.D9.enable();
        delay.delay_ms(10u32);
        chip.leds.D9.disable();
    }
}
