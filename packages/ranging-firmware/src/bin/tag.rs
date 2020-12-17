#![no_std]
#![no_main]

// use dwm1001::nrf52832_hal::{prelude::*, timer, Timer};
// use dwm1001::{dw1000::DW1000, nrf52832_hal as hal};
// use dwm1001::{nrf52832_hal::gpio::Level, DWM1001};

use defmt::{debug, info, warn};
use defmt_rtt as _;
use dw1000::{
    mac,
    ranging::{self, Message as _},
    RxConfig, TxConfig,
};
// global logger
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use hal::prelude::*;
use hal::{gpio::Level, Delay, Timer};
use mobile_firmware::{block_timeout, DWM1001};
use nrf52832_hal as hal;
use panic_probe as _;

static OUT: &[u8] = b"PING";
// static OUT: &[u8] = b"PING\r\n";

#[cortex_m_rt::entry]
fn main() -> ! {
    // debug::init();

    let mut dwm1001 = DWM1001::take().unwrap();
    let mut delay = Delay::new(dwm1001.SYST);

    dwm1001.DW_RST.reset_dw1000(&mut delay);
    let mut dw1000 = dwm1001.DW1000.init().expect("Failed to initialize DW1000");

    // Configure timer
    let mut timer = Timer::new(dwm1001.TIMER0);

    loop {
        defmt::info!("receinv started!");
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
                // print!("Error: {:?}\n", error);
                continue;
            }
        };

        if message.frame.payload.starts_with(ranging::Ping::PRELUDE.0) {
            // dwm1001.leds.D10.enable();
            delay.delay_ms(10u32);
            // dwm1001.leds.D10.disable();
            continue;
        }
        if message
            .frame
            .payload
            .starts_with(ranging::Request::PRELUDE.0)
        {
            // dwm1001.leds.D11.enable();
            delay.delay_ms(10u32);
            // dwm1001.leds.D11.disable();
            continue;
        }
        if message
            .frame
            .payload
            .starts_with(ranging::Response::PRELUDE.0)
        {
            // dwm1001.leds.D12.enable();
            delay.delay_ms(10u32);
            // dwm1001.leds.D12.disable();
            continue;
        }

        // dwm1001.leds.D9.enable();
        delay.delay_ms(10u32);
        defmt::info!("Frame received!");
        // dwm1001.leds.D9.disable();

        // print!("Received frame: {:x?}\n", message.frame);
    }
}

// #[cortex_m_rt::entry]
// fn main() -> ! {
//     // let board = mobile_firmware::Board::init();

//     // let p = hal::pac::Peripherals::take().unwrap();
//     // let port0 = hal::gpio::p0::Parts::new(p.P0);
//     // let button = port0.p0_13.into_pullup_input();
//     // let mut led = port0.p0_12.into_push_pull_output(Level::Low);

//     let mut dwm = DWM1001::take().unwrap();
//     let mut timer = Timer::new(dwm.TIMER0);
//     let mut delay = Delay::new(dwm.SYST);
//     // let mut timer = Timer::new(dwm.TIMER0);
//     let mut led = dwm.pins.GPIO_12.into_push_pull_output(Level::Low);
//     dwm.DW_RST.reset_dw1000(&mut delay);
//     let mut dw1000 = dwm.DW1000.init().unwrap();

//     // Configure timer

//     loop {
//         delay.delay_ms(200u32);
//         led.set_high();
//         let mut receiving = dw1000
//             .receive(RxConfig {
//                 frame_filtering: false,
//                 ..RxConfig::default()
//             })
//             .expect("Failed to start receiver");

//         let mut buffer = [0; 1024];

//         // Set timer for timeout
//         timer.start(5_000_000u32);

//         let result = block_timeout!(&mut timer, receiving.wait(&mut buffer));
//         led.set_low();

//         dw1000 = receiving
//             .finish_receiving()
//             .expect("Failed to finish receiving");

//         let message = match result {
//             Ok(message) => message,
//             Err(error) => {
//                 defmt::warn!("error!");
//                 match error {
//                     embedded_timeout_macros::TimeoutError::Timeout => {
//                         warn!("timeout occured");
//                         led.set_low();
//                     }
//                     embedded_timeout_macros::TimeoutError::Other(f) => {
//                         warn!("other error occured");
//                         match f {
//                             dw1000::Error::Spi(_) => {
//                                 warn!("error: Spi");
//                             }
//                             dw1000::Error::Fcs => {
//                                 warn!("error: Fcs");
//                             }
//                             dw1000::Error::Phy => {
//                                 warn!("error: Phy");
//                             }
//                             dw1000::Error::BufferTooSmall { required_len } => {
//                                 warn!("error: BufferTooSmall");
//                             }
//                             dw1000::Error::ReedSolomon => {
//                                 warn!("error: ReedSolomon");
//                             }
//                             dw1000::Error::FrameWaitTimeout => {
//                                 warn!("error: FrameWaitTimeout");
//                             }
//                             dw1000::Error::Overrun => {
//                                 warn!("error: Overrun");
//                             }
//                             dw1000::Error::PreambleDetectionTimeout => {
//                                 warn!("error: PreambleDetectionTimeout");
//                             }
//                             dw1000::Error::SfdTimeout => {
//                                 warn!("error: SfdTimeout");
//                             }
//                             dw1000::Error::FrameFilteringRejection => {
//                                 warn!("error: FrameFilteringRejection");
//                             }
//                             dw1000::Error::Frame(_) => {
//                                 warn!("error: Frame");
//                             }
//                             dw1000::Error::DelayedSendTooLate => {
//                                 warn!("error: DelayedSendTooLate");
//                             }
//                             dw1000::Error::DelayedSendPowerUpWarning => {
//                                 warn!("error: DelayedSendPowerUpWarning");
//                             }
//                             dw1000::Error::Ssmarshal(_) => {
//                                 warn!("error: Ssmarshal");
//                             }
//                             dw1000::Error::InvalidConfiguration => {
//                                 warn!("error: InvalidConfiguration");
//                             }
//                         }
//                     }
//                 }
//                 // print!("Error: {:?}\n", error);
//                 continue;
//             }
//         };
//         defmt::info!("hell");

//         if message.frame.payload.starts_with(ranging::Ping::PRELUDE.0) {
//             // dwm1001.leds.D10.enable();

//             delay.delay_ms(10u32);
//             // dwm1001.leds.D10.disable();
//             info!("Message received!1");
//             continue;
//         }
//         if message
//             .frame
//             .payload
//             .starts_with(ranging::Request::PRELUDE.0)
//         {
//             // dwm1001.leds.D11.enable();
//             delay.delay_ms(10u32);
//             // dwm1001.leds.D11.disable();
//             info!("Message received!2");
//             continue;
//         }
//         if message
//             .frame
//             .payload
//             .starts_with(ranging::Response::PRELUDE.0)
//         {
//             // dwm1001.leds.D12.enable();
//             delay.delay_ms(10u32);
//             // dwm1001.leds.D12.disable();
//             info!("Message received!3");
//             // info!("Message received!");
//             continue;
//         }

//         led.set_high();
//         defmt::info!("uh");
//         // dwm1001.leds.D9.enable();
//         delay.delay_ms(500u32);

//         led.set_low();

//         // dwm1001.leds.D9.disable();
//         // print!("Received frame: {:x?}\n", message.frame);
//     }

//     loop {
//         // led.set_high().unwrap();
//         // let mut sending = dw1000
//         //     .send(
//         //         b"ping",
//         //         mac::Address::broadcast(&mac::AddressMode::Short),
//         //         None,
//         //         TxConfig::default(),
//         //     )
//         //     .expect("Failed to start receiver");

//         // block!(sending.wait()).expect("Failed to send data");

//         // dw1000 = sending.finish_sending().expect("Failed to finish sending");
//         // led.set_low().unwrap();

//         // delay(&mut timer, 500_000); // 20ms
//         //
//         //
//         // led.set_low().unwrap();
//         // delay(&mut timer, 500_000); // 20ms

//         // match dwm.uart.write(&b) {
//         //     Ok(_) => {}
//         //     Err(e) => defmt::info!("writing to uart failed"),
//         // }
//     }
// }

// use nb::block;
// fn delay<T>(timer: &mut Timer<T>, cycles: u32)
// where
//     T: hal::timer::Instance,
// {
//     timer.start(cycles);
//     block!(timer.wait()).unwrap();
// }
