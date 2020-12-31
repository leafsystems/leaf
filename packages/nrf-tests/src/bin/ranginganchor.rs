#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use defmt_rtt as _;
use nb::block;

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
use nrf_tests as _;

#[entry]
fn main() -> ! {
    /*
        Configure the DWM module
    */
    let mut dwm = DWM1001::take().unwrap();

    let mut delay = Delay::new(dwm.SYST);
    let mut rng = Rng::new(dwm.RNG);

    dwm.DW_RST.reset_dw1000(&mut delay);
    let mut dw = dwm.DW1000.init().expect("Failed to initialize DW1000");

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

    // let mut task_timer = Timer::new(dwm.TIMER0);
    let mut timeout_timer = Timer::new(dwm.TIMER1);

    // task_timer.start(1_000_000u32);

    let mut buf = [0; 128];

    loop {
        led.set_high();
        delay.delay_ms(100u32);
        led.set_low();
        // After receiving for a while, it's time to send out a ping
        // if let Ok(()) = task_timer.wait() {
        //     task_timer.start(5_000_000u32);

        //     dwm.leds.D10.enable();
        //     delay.delay_ms(10u32);
        //     dwm.leds.D10.disable();

        //     let mut sending = ranging::Ping::new(&mut dw)
        //         .expect("Failed to initiate ping")
        //         .send(dw)
        //         .expect("Failed to initiate ping transmission");

        //     timeout_timer.start(100_000u32);
        //     // dw_irq.wait_for_interrupts(&mut gpiote, &mut timeout_timer);
        //     block_timeout!(&mut timeout_timer, sending.wait())
        //         // block!({
        //         //     dw_irq.wait_for_interrupts(&mut gpiote, &mut timeout_timer);
        //         //     sending.wait()
        //         // })
        //         .expect("Failed to send ping");

        //     dw = sending.finish_sending().expect("Failed to finish sending");
        // }

        let mut receiving = dw
            .receive(RxConfig {
                frame_filtering: false,
                ..RxConfig::default()
            })
            .expect("Failed to receive message");

        timeout_timer.start(500_000u32);
        let result = block_timeout!(&mut timeout_timer, {
            dw_irq.wait_for_interrupts(&mut gpiote, &mut timeout_timer);
            receiving.wait(&mut buf)
        });

        dw = receiving
            .finish_receiving()
            .expect("Failed to finish receiving");

        let message = match result {
            Ok(message) => message,
            _ => continue,
        };

        led.set_high();
        delay.delay_ms(10u32);
        led.set_low();

        let request = ranging::Request::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>(&message);

        let request = match request {
            Ok(Some(request)) => request,
            Ok(None) | Err(_) => {
                info!("Ignoring message that is not a request\n");
                continue;
            }
        };

        led.set_high();
        delay.delay_ms(10u32);
        led.set_low();

        // Wait for a moment, to give the tag a chance to start listening for
        // the reply.
        delay.delay_ms(10u32);

        // Send ranging response
        let mut sending = ranging::Response::new(&mut dw, &request)
            .expect("Failed to initiate response")
            .send(dw)
            .expect("Failed to initiate response transmission");
        timeout_timer.start(100_000u32);
        block!({
            dw_irq.wait_for_interrupts(&mut gpiote, &mut timeout_timer);
            sending.wait()
        })
        .expect("Failed to send ranging response");

        dw = sending.finish_sending().expect("Failed to finish sending");
        led.set_high();
        delay.delay_ms(10u32);
        led.set_low();
    }
}
