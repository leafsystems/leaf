//! Listens for for incoming transmissions, logging anything it reads.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::info;
use dwm1001::{
    block_timeout, debug,
    dw1000::{mac, RxConfig, TxConfig},
    nrf52832_hal::{gpio::Level, Delay, Timer},
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
    let mut dw1000 = dwm1001.DW1000.init().unwrap();
    let mut led = dwm1001.pins.GPIO_12.into_push_pull_output(Level::Low);
    let mut delay = Delay::new(dwm1001.SYST);
    dwm1001.DW_RST.reset_dw1000(&mut delay);

    let mut timer = Timer::new(dwm1001.TIMER0);

    loop {
        // info!("Receiving started");
        let mut receiving = dw1000
            .receive(RxConfig {
                frame_filtering: false,
                ..RxConfig::default()
            })
            .expect("Failed to start receiver");

        let mut buffer = [0; 1024];

        // info!("receivng buffer made");
        // Set timer for timeout
        // timer.start(5_000_000u32);
        // timer.start(5_000_000u32);

        let result = block!(receiving.wait(&mut buffer));
        // info!("result made");
        // let result = block!(&mut timer, receiving.wait(&mut buffer));
        // let result = block_timeout!(&mut timer, receiving.wait(&mut buffer));

        dw1000 = receiving
            .finish_receiving()
            .expect("Failed to finish receiving");

        let message = match result {
            Ok(message) => {
                info!("No error occured");
                message
            }
            Err(error) => {
                match error {
                    dwm1001::dw1000::Error::Spi(_) => {
                        info!("error: Spi")
                    }
                    dwm1001::dw1000::Error::Fcs => {
                        info!("error: Fcs")
                    }
                    dwm1001::dw1000::Error::Phy => {
                        info!("error: Phy")
                    }
                    dwm1001::dw1000::Error::BufferTooSmall { required_len } => {
                        info!("error: BufferTooSmall")
                    }
                    dwm1001::dw1000::Error::ReedSolomon => {
                        info!("error: ReedSolomon")
                    }
                    dwm1001::dw1000::Error::FrameWaitTimeout => {
                        info!("error: FrameWaitTimeout")
                    }
                    dwm1001::dw1000::Error::Overrun => {
                        info!("error: Overrun")
                    }
                    dwm1001::dw1000::Error::PreambleDetectionTimeout => {
                        info!("error: PreambleDetectionTimeout")
                    }
                    dwm1001::dw1000::Error::SfdTimeout => {
                        info!("error: SfdTimeout")
                    }
                    dwm1001::dw1000::Error::FrameFilteringRejection => {
                        info!("error: FrameFilteringRejection")
                    }
                    dwm1001::dw1000::Error::Frame(_) => {
                        info!("error: Frame")
                    }
                    dwm1001::dw1000::Error::DelayedSendTooLate => {
                        info!("error: DelayedSendTooLate")
                    }
                    dwm1001::dw1000::Error::DelayedSendPowerUpWarning => {
                        info!("error: DelayedSendPowerUpWarning")
                    }
                    dwm1001::dw1000::Error::Ssmarshal(_) => {
                        info!("error: Ssmarshal")
                    }
                    dwm1001::dw1000::Error::InvalidConfiguration => {
                        info!("error: InvalidConfiguration")
                    }
                }
                // info!("An error occured");
                // info!("Error: {:?}\n", error);
                // info!("Error: {:?}\n", error);
                continue;
            }
        };

        led.set_high();
        delay.delay_ms(50u32);
        led.set_low();
        delay.delay_ms(50u32);
        // print!(".");
        defmt::info!("received!");
    }
}
