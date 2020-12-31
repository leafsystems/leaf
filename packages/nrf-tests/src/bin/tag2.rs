//! Wait for a message, sending a response when we get pinged

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use dwm1001::{
    block_timeout, debug,
    dw1000::{
        mac,
        ranging::{self, Message},
        RxConfig, TxConfig,
    },
    nrf52832_hal::{
        gpio::{p0::P0_17, Level, Output, PushPull},
        pac::SPIM2,
        Delay, Spim, Timer,
    },
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
            mac::PanId(0x0d57),        // hardcoded network id
            mac::ShortAddress(0x0002), // random device address
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

        // Receive a message
        info!("Receiving message");
        let mut receiving = dw1000
            .receive(RxConfig {
                frame_filtering: false,
                ..RxConfig::default()
            })
            .expect("Failed to start receiver");

        let result = block!(receiving.wait(&mut buf));
        dw1000 = receiving
            .finish_receiving()
            .expect("Failed to finish sending");

        match result {
            Ok(message) => {
                let g = message.frame.payload;
                info!("message is {:?}", g);
                let request =
                    ranging::Request::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>(&message);
                if let Ok(Some(request)) = request {
                    led.set_high();
                    let mut sending = ranging::Response::new(&mut dw1000, &request)
                        .expect("Failed to initiate response")
                        .send(dw1000)
                        .expect("Failed to initiate response transmission");
                    block!(sending.wait()).expect("Failed to send data");
                    dw1000 = sending.finish_sending().expect("Failed to finish sending");
                    delay.delay_ms(50u32);
                    led.set_low();
                    info!("Successfully replied");
                } else {
                    info!("not a ranging message")
                }
            }
            Err(_) => {
                info!("An error occurred receiving");
            }
        }
    }
}
