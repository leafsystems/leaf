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

use defmt_rtt as _;
use nrf52832_hal::{
    gpio::{p0::P0_17, Output, PushPull},
    pac::SPIM2,
    Spim,
};
use panic_probe as _;

use dwm1001::{
    block_timeout,
    dw1000::{mac, ranging},
    nrf52832_hal::{rng::Rng, Delay, Timer},
    prelude::*,
};
use uart_types::DataReading;

type Spi1 = Spim<SPIM2>;
type Spi2 = P0_17<Output<PushPull>>;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::debug!("Launching anchor");

    let mut chip = dwm1001::DWM1001::take().unwrap();
    let mut delay = Delay::new(chip.SYST);
    let mut rng = Rng::new(chip.RNG);
    let mut timer = Timer::new(chip.TIMER0);
    let our_addr = rng.random_u16();

    let mut radio = disc::build_radio(
        &mut chip.DW_RST,
        chip.DW1000,
        &mut delay,
        our_addr,
        disc::UwbPerformance::Medium,
    );

    let mut radio_buf = [0u8; 1024];
    let mut uart_buf = [0u8; core::mem::size_of::<DataReading>()];

    loop {
        // // requires multiple packets

        let mut receiving = radio.receive().expect("Failed to receive message");

        let msg = nb::block!(receiving.wait(&mut radio_buf));

        match msg.map(|msg| msg.decode::<ranging::Ping, Spi1, Spi2>()) {
            Ok(Ok(Some(ping))) => {
                let mut sending = ranging::Request::new(&mut radio, &ping)
                    .unwrap()
                    .send(&mut radio)
                    .unwrap();

                nb::block!(sending.wait_transmit()).unwrap();

                let mut receiving = radio.receive().expect("Failed to receive message");

                // Set timer for timeout
                timer.start(5_000_000u32);

                let msg = block_timeout!(&mut timer, receiving.wait(&mut radio_buf));

                if let Ok(Ok(Some(response))) =
                    msg.map(|m| m.decode::<ranging::Response, Spi1, Spi2>())
                {
                    //
                    // If this is not a PAN ID and short address, it doesn't
                    // come from a compatible node. Ignore it.
                    let (pan_id, addr) = match response.source {
                        Some(mac::Address::Short(pan_id, addr)) => (pan_id, addr),
                        _ => continue,
                    };

                    // Ranging response received. Compute distance.
                    if let Ok(distance_mm) = ranging::compute_distance_mm(&response) {
                        chip.leds.D9.enable();
                        delay.delay_ms(10u32);
                        chip.leds.D9.disable();

                        defmt::debug!("{:04x}:{:04x} - {} mm", pan_id.0, addr.0, distance_mm,);

                        let msg = DataReading {
                            anchor: our_addr,
                            distance_mm,
                            timestamp: response.rx_time.value(),
                            tag: addr.0,
                        };

                        use zerocopy::AsBytes;
                        let bytes = msg.as_bytes();

                        // .write(postcard::to_slice(&msg, &mut uart_buf).unwrap())
                        if chip.uart.write(bytes).is_err() {
                            defmt::error!("Failed to write to uart");
                        };
                    }
                }
            }
            _ => continue,
        }
    }
}
