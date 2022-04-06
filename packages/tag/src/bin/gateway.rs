//! This a very special type of anchor that acts as a local gateway. It manages
//! local anchors and rolls their data up to the cloud over a uart connection during downtime.
//!
//! The accompanying anchors are much more "dumb" - they simply listen for the
//! echos of the ranging dances between the gateway and the tags. This data is
//! reported back later and used to perform full localization
//!
//! This lets us scale to a large number of moving tags and keep the anchor infrastructure a bit more minimal.
//! We're mostly concerned with the tags being as "dumb" as possible - moving from
//! zone to zone without an idea of the infrastructure that they're contained within.

#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

use lis2dh12::{Lis2dh12, RawAccelerometer};
use tag::{configure_rx, Msg};

use dwm1001::{
    block_timeout,
    dw1000::{
        mac,
        ranging::{self, compute_distance_mm, Message as _RangingMessage},
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

use uart_types::{DataReading, GatewayCommand};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::debug!("Launching gateway anchor");

    let mut chip = dwm1001::DWM1001::take().unwrap();
    let mut delay = Delay::new(chip.SYST);
    let mut rng = Rng::new(chip.RNG);
    let our_addr = rng.random_u16();

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

        radio
            .set_antenna_delay(16456, 16300)
            .expect("Failed to set antenna delay");

        // Set network address
        radio
            .set_address(
                mac::PanId(0x0d57),          // hardcoded network id
                mac::ShortAddress(our_addr), // random device address
            )
            .expect("Failed to set address");

        radio
    };

    let mut timeout_timer = Timer::new(chip.TIMER0);
    let mut dw_irq = chip.DW_IRQ;
    let mut gpiote = chip.GPIOTE;
    let mut buf = [0u8; 1024];
    let mut uart_buf = [0u8; core::mem::size_of::<DataReading>()];

    loop {
        // Set timer for timeout
        timeout_timer.start(5_000_000u32);

        // Wait for the ranging request
        let mut receiving = radio
            .receive(configure_rx())
            .expect("Failed to receive message");

        let result = block_timeout!(&mut timeout_timer, receiving.wait_receive(&mut buf));

        radio = receiving
            .finish_receiving()
            .expect("Failed to finish receiving");

        let ping = match result
            .as_ref()
            .map(ranging::Ping::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>)
        {
            Ok(Ok(Some(request))) => request,
            e => continue,
        };

        // Return with a ranging request
        let mut sending = ranging::Request::new(&mut radio, &ping)
            .expect("Failed to initiate response")
            .send(radio, tag::configure_tx())
            .expect("Failed to initiate response transmission");

        timeout_timer.start(100_000u32);

        nb::block!(sending.wait_transmit()).expect("Failed to send ranging response");

        radio = sending.finish_sending().expect("Failed to finish sending");

        // Wait for the ranging response
        let mut receiving = radio
            .receive(configure_rx())
            .expect("Failed to receive message");

        let result = block_timeout!(&mut timeout_timer, receiving.wait_receive(&mut buf));

        radio = receiving
            .finish_receiving()
            .expect("Failed to finish receiving");

        let response = match result
            .as_ref()
            .map(ranging::Response::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>)
        {
            Ok(Ok(Some(request))) => request,
            _ => continue,
        };

        // Received ranging response from anchor. Now we can compute the
        // distance.

        chip.leds.D11.enable();
        delay.delay_ms(10u32);
        chip.leds.D11.disable();

        // If this is not a PAN ID and short address, it doesn't
        // come from a compatible node. Ignore it.
        let (pan_id, addr) = match response.source {
            Some(mac::Address::Short(pan_id, addr)) => (pan_id, addr),
            _ => continue,
        };

        // Ranging response received. Compute distance.
        if let Ok(distance_mm) = ranging::compute_distance_mm(&response, configure_rx()) {
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

            if chip
                .uart
                .write(postcard::to_slice(&msg, &mut uart_buf).unwrap())
                .is_err()
            {
                defmt::error!("Failed to write to uart");
            };
        }
    }
}
