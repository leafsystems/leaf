#![no_std]
#![no_main]

use cortex_m_rt::entry;

use dwm1001::{
    block_timeout, debug,
    dw1000::{
        ranging::{self, Message as _},
        RxConfig,
    },
    nrf52832_hal::{gpio::Level, Delay, Timer},
    prelude::*,
    print, DWM1001,
};
use nrf_tests as _;

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
// use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // debug::init();

    let mut dwm1001 = DWM1001::take().unwrap();
    let mut delay = Delay::new(dwm1001.SYST);

    dwm1001.DW_RST.reset_dw1000(&mut delay);
    let mut dw1000 = dwm1001.DW1000.init().expect("Failed to initialize DW1000");

    // Configure timer
    let mut timer = Timer::new(dwm1001.TIMER0);
    let mut led = dwm1001.pins.GPIO_12.into_push_pull_output(Level::Low);

    loop {
        let mut receiving = dw1000
            .receive(RxConfig {
                frame_filtering: false,
                ..RxConfig::default()
            })
            .expect("Failed to start receiver");

        let mut buffer = [0; 1024];

        // Set timer for timeout
        timer.start(5_000_000u32);

        let result = block_timeout!(&mut timer, receiving.wait(&mut buffer));

        dw1000 = receiving
            .finish_receiving()
            .expect("Failed to finish receiving");

        let message = match result {
            Ok(message) => message,
            Err(error) => {
                print!("Error: {:?}\n", error);
                continue;
            }
        };

        if message.frame.payload.starts_with(ranging::Ping::PRELUDE.0) {
            led.set_high();
            delay.delay_ms(10u32);
            led.set_low();

            continue;
        }
        if message
            .frame
            .payload
            .starts_with(ranging::Request::PRELUDE.0)
        {
            led.set_high();
            delay.delay_ms(10u32);
            led.set_low();

            continue;
        }
        if message
            .frame
            .payload
            .starts_with(ranging::Response::PRELUDE.0)
        {
            led.set_high();
            delay.delay_ms(10u32);
            led.set_low();

            continue;
        }

        led.set_high();
        delay.delay_ms(10u32);
        led.set_low();

        defmt::info!("Received frame!");
        // message.frame.payload
        defmt::info!("{:?}!", message.frame.payload);

        // let b = b
        // match dwm1001.uart.write(message.frame.payload) {
        //     Ok(_) => {}
        //     Err(e) => defmt::info!("writing to uart failed"),
        // }
        // defmt::info!("{}!", message.);
        // print!("Received frame: {:x?}\n", message.frame);
    }
}
