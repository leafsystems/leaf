#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use nrf_tests as _;

use dwm1001::{
    block_timeout, debug,
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
    print, DWM1001,
};
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    /*
        Configure the DWM module
    */

    let mut dwm = DWM1001::take().unwrap();

    let mut delay = Delay::new(dwm.SYST);
    let mut rng = Rng::new(dwm.RNG);

    dwm.DW_RST.reset_dw1000(&mut delay);
    let mut dw = dwm.DW1000.init().expect("Failed to initialize dw");

    dw.enable_tx_interrupts()
        .expect("Failed to enable TX interrupts");
    dw.enable_rx_interrupts()
        .expect("Failed to enable RX interrupts");

    let mut dw_irq = dwm.DW_IRQ;
    let mut gpiote = dwm.GPIOTE;

    let mut led = dwm.pins.GPIO_12.into_push_pull_output(Level::Low);
    // These are the hardcoded calibration values from the dwm1001-examples
    // repository[1]. Ideally, the calibration values would be determined using
    // the proper calibration procedure, but hopefully those are good enough for
    // now.
    //
    // [1] https://github.com/Decawave/dwm1001-examples
    dw.set_antenna_delay(16456, 16300)
        .expect("Failed to set antenna delay");

    // Set network address
    dw.set_address(
        mac::PanId(0x0d57),                  // hardcoded network id
        mac::ShortAddress(rng.random_u16()), // random device address
    )
    .expect("Failed to set address");

    let mut timeout_timer = Timer::new(dwm.TIMER1);

    let mut buf = [0; 128];

    loop {
        led.set_high();
        delay.delay_ms(100u32);
        led.set_low();

        info!("starting receive.");
        let mut receiving = dw
            .receive(RxConfig {
                frame_filtering: false,
                ..RxConfig::default()
            })
            // .receive(RxConfig::default())
            .expect("Failed to receive message");

        timeout_timer.start(500_000u32);
        // timeout_timer.start(500_000u32);
        // dw_irq.wait_for_interrupts(&mut gpiote, &mut timeout_timer);
        let message = block_timeout!(&mut timeout_timer, receiving.wait(&mut buf));
        // info!("message received?.");

        dw = receiving
            .finish_receiving()
            .expect("Failed to finish receiving");
        // info!("Finished receiving");

        let message = match message {
            Ok(message) => message,
            Err(_) => continue, //ignore error
        };
        // info!("Decoding message...");

        let ping = ranging::Ping::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>(&message)
            .expect("Failed to decode ping");
        if let Some(ping) = ping {
            // Received ping from an anchor. Reply with a ranging
            // request.

            // led.set_high();
            // delay.delay_ms(10u32);
            // led.set_low();

            // Wait for a moment, to give the anchor a chance to start listening
            // for the reply.
            delay.delay_ms(10u32);

            let mut sending = ranging::Request::new(&mut dw, &ping)
                .expect("Failed to initiate request")
                .send(dw)
                .expect("Failed to initiate request transmission");

            timeout_timer.start(500_000u32);
            block_timeout!(&mut timeout_timer, {
                dw_irq.wait_for_interrupts(&mut gpiote, &mut timeout_timer);
                sending.wait()
            })
            .expect("Failed to send ranging request");

            dw = sending.finish_sending().expect("Failed to finish sending");

            continue;
        }

        let response = ranging::Response::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>(&message)
            .expect("Failed to decode response");
        if let Some(response) = response {
            // Received ranging response from anchor. Now we can compute the
            // distance.

            led.set_high();
            delay.delay_ms(10u32);
            led.set_low();

            // If this is not a PAN ID and short address, it doesn't
            // come from a compatible node. Ignore it.
            let (pan_id, addr) = match response.source {
                mac::Address::Short(pan_id, addr) => (pan_id, addr),
                _ => continue,
            };

            // Ranging response received. Compute distance.
            let distance_mm = ranging::compute_distance_mm(&response).unwrap();

            led.set_high();
            delay.delay_ms(10u32);
            led.set_low();

            defmt::info!("Distance found! {:?}", distance_mm);
            // print!("{:04x}:{:04x} - {} mm\n", pan_id.0, addr.0, distance_mm,);

            continue;
        } else {
            info!("Failed to decode response ");
        }

        info!("Ignored message ");
    }
}
