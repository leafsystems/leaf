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

use lis2dh12::RawAccelerometer;

use defmt_rtt as _;
use tag::{configure_rx, configure_tx, Msg};

use nrf52832_hal::{
    gpio::p0::{self},
    Twim,
};
use panic_probe as _;

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

static ID: Option<&str> = core::option_env!("BASE_STATION_ID");

#[cortex_m_rt::entry]
fn main() -> ! {
    let _our_id: u8 = ID.unwrap_or("0").parse().unwrap();

    defmt::debug!("Launching basestation");

    let mut dwm = dwm1001::DWM1001::take().unwrap();

    let mut delay = Delay::new(dwm.SYST);
    let mut rng = Rng::new(dwm.RNG);

    dwm.DW_RST.reset_dw1000(&mut delay);
    let mut radio = dwm
        .DW1000
        .init(&mut delay)
        .expect("Failed to initialize DW1000");

    radio
        .enable_tx_interrupts()
        .expect("Failed to enable TX interrupts");
    radio
        .enable_rx_interrupts()
        .expect("Failed to enable RX interrupts");

    // These are the hardcoded calibration values from the dwm1001-examples
    // repository[1]. Ideally, the calibration values would be determined using
    // the proper calibration procedure, but hopefully those are good enough for
    // now.
    //
    // [1] https://github.com/Decawave/dwm1001-examples
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

    let address = lis2dh12::SlaveAddr::Alternative(true);

    let mut lis2dh12 = lis2dh12::Lis2dh12::new(dwm.LIS2DH12, address).expect("lis2dh12 new failed");

    defmt::info!(
        "WHOAMI: {:08b}",
        lis2dh12
            .get_device_id()
            .expect("lis2dh12 get_device_id failed")
    );

    lis2dh12
        .set_mode(lis2dh12::Mode::HighResolution)
        .expect("lis2dh12 set_mode failed");

    lis2dh12
        .set_odr(lis2dh12::Odr::Hz1)
        .expect("lis2dh12 set_odr failed");

    lis2dh12
        .enable_axis((true, true, true))
        .expect("lis2dh12 enable_axis failed");

    lis2dh12
        .enable_temp(true)
        .expect("lis2dh2 enable_temp failed");

    // let twim = make_gryo(&dwm);

    // let scl3 = dwm
    //     .pc0
    //     .into_open_drain_output(&mut gpioc.moder, &mut gpioc.otyper)
    //     .into_af4(&mut gpioc.moder, &mut gpioc.afrl);

    // let sda3 = gpioc
    //     .pc1
    //     .into_open_drain_output(&mut gpioc.moder, &mut gpioc.otyper)
    //     .into_af4(&mut gpioc.moder, &mut gpioc.afrl);

    // let i2c = I2c::i2c3(
    //     device.I2C3,
    //     (scl3, sda3),
    //     KiloHertz(100),
    //     clocks,
    //     &mut rcc.apb1r1,
    // );

    let mut timer = Timer::new(dwm.TIMER0);

    let mut buffer1 = [0; 1024];
    let mut buffer2 = [0; 1024];

    let mut temp = Temp::new(dwm.TEMP);

    let mut uart_buf = [0u8; 24];

    loop {
        /*
        - wait for ping
        - initiate ranging request
        - wait for response
        - calculate distance
        - log it
        */

        defmt::debug!("waiting for base mobile tag ping");

        let mut receiving = radio
            .receive(configure_rx())
            .expect("Failed to receive message");

        let message = block_timeout!(&mut timer, receiving.wait_receive(&mut buffer1));

        radio = receiving
            .finish_receiving()
            .expect("Failed to finish receiving");

        let message = match message {
            Ok(message) => message,
            Err(_) => {
                defmt::error!("Timeout error occured");
                continue;
            }
        };

        let ping = match ranging::Ping::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>(&message) {
            Ok(Some(ping)) => ping,
            Ok(None) => {
                defmt::error!("Failed to decode ping");
                continue;
            }
            Err(e) => {
                defmt::error!("Ping decode error: {:?}", defmt::Debug2Format(&e));
                continue;
            }
        };

        // Received ping from an anchor. Reply with a ranging
        defmt::debug!("Received ping. Responding with ranging request.");

        dwm.leds.D10.enable();
        delay.delay_ms(10u32);
        dwm.leds.D10.disable();

        // Wait for a moment, to give the anchor a chance to start listening
        // for the reply.
        delay.delay_ms(10u32);

        let mut sending = ranging::Request::new(&mut radio, &ping)
            .expect("Failed to initiate request")
            .send(radio, configure_tx())
            .expect("Failed to initiate request transmission");

        nb::block!(sending.wait_transmit()).expect("Failed to send data");
        radio = sending.finish_sending().expect("Failed to finish sending");

        defmt::debug!("Request sent Transmission sent. Waiting for response.");

        let mut receiving = radio
            .receive(configure_rx())
            .expect("Failed to receive message");

        // Set timer for timeout
        timer.start(5_000_000u32);
        let result = block_timeout!(&mut timer, receiving.wait_receive(&mut buffer2));

        radio = receiving
            .finish_receiving()
            .expect("Failed to finish receiving");

        let message = match result {
            Ok(message) => message,
            Err(error) => {
                use embedded_timeout_macros::TimeoutError;
                match error {
                    TimeoutError::Timeout => {
                        defmt::debug!("Waiting for base station timed out. Trying again.")
                    }
                    TimeoutError::Other(o) => {
                        defmt::error!("Other error: {:?}", defmt::Debug2Format(&o));
                    }
                }
                continue;
            }
        };

        let response =
            match ranging::Response::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>(&message) {
                Ok(Some(response)) => response,
                Ok(None) => {
                    let p = message.frame.payload;
                    defmt::error!(
                        "Failed to decode.
                        Found:    {=[u8]}
                        Expected: {=[u8]}",
                        *p,
                        *ranging::Response::PRELUDE.0
                    );
                    // Frame {
                    //     header: Header {
                    //         frame_type: Data,
                    //         frame_pending: false,
                    //         ack_request: false,
                    //         pan_id_compress: false,
                    //         seq_no_suppress: false,
                    //         ie_present: false,
                    //         version: Ieee802154_2006,
                    //         seq: 129,
                    //         destination: Some(Short(PanId(65535), ShortAddress(65535))),
                    //         source: Some(Short(PanId(3415), ShortAddress(6325))),
                    //         auxiliary_security_header: None,
                    //     },
                    //     content: Data,
                    //     payload: [
                    //         82, 65, 78, 71, 73, 78, 71, 32, 80, 73, 78, 71, 172, 91, 228, 168, 99,
                    //         0, 0, 0,
                    //     ],
                    //     footer: [189, 146],
                    // };

                    defmt::error!(
                        "Failed to decode ranging response. Frame is {:?}",
                        defmt::Debug2Format(&message.frame)
                    );
                    continue;
                }
                Err(e) => {
                    defmt::error!(
                        "Ranging response decode error: {:?}",
                        defmt::Debug2Format(&e)
                    );
                    continue;
                }
            };

        dwm.leds.D11.enable();
        delay.delay_ms(10u32);
        dwm.leds.D11.disable();

        // If this is not a PAN ID and short address, it doesn't
        // come from a compatible node. Ignore it.
        let (pan_id, addr) = match response.source {
            Some(mac::Address::Short(pan_id, addr)) => (pan_id, addr),
            _ => continue,
        };

        // Ranging response received. Compute distance.
        match ranging::compute_distance_mm(&response, RxConfig::default()) {
            Ok(distance_mm) => {
                dwm.leds.D9.enable();
                delay.delay_ms(10u32);
                dwm.leds.D9.disable();

                let temp = temp.measure().to_num();
                let accel = lis2dh12.accel_raw().unwrap();

                let lis2dh12::I16x3 { x, y, z } = accel;

                let msg = Msg {
                    id: 0,
                    temp,
                    accel: (x, y, z),
                    distance: distance_mm,
                };

                postcard::to_slice(&msg, &mut uart_buf).unwrap();

                dwm.uart.write(&uart_buf).unwrap();

                defmt::info!(
                    "\ntag: {:04x}:{:04x}\ntemp: {:?}\nrange: {} mm",
                    pan_id.0,
                    addr.0,
                    msg.temp,
                    distance_mm,
                );
            }
            Err(e) => {
                defmt::error!("Ranging response error: {:?}", defmt::Debug2Format(&e));
            }
        }
    }
}
use nrf52832_hal::pac;
use nrf52832_hal::twim;

fn make_gryo() -> Twim<pac::TWIM0> {
    let p = pac::Peripherals::take().unwrap();
    let port0 = p0::Parts::new(p.P0);

    let scl = port0.p0_26.into_floating_input().degrade();
    let sda = port0.p0_27.into_floating_input().degrade();

    let pins = twim::Pins { scl, sda };

    let i2c = Twim::new(p.TWIM0, pins, twim::Frequency::K100);

    i2c

    // let p0 = Parts::new(dwm.pins.P0);

    // let scl = p0.p0_30.into_floating_input().degrade();
    // let sda = p0.p0_31.into_floating_input().degrade();

    // Twim::new(
    //     dwm.TWI1,
    //     twim::Pins {
    //         scl: dwm.pins.scl.into_floating_input().degrade(),
    //         sda: sda.into_floating_input().degrade(),
    //     },
    //     twim::Frequency::K250,
    // )
}
