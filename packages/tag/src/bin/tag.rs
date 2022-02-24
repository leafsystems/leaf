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

use lis2dh12::{Lis2dh12, RawAccelerometer};

use defmt_rtt as _;
use tag::Msg;

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
use uart_types::{DataReading, DATA_BUF_SIZE};

static ID: Option<&str> = core::option_env!("BASE_STATION_ID");

#[cortex_m_rt::entry]
fn main() -> ! {
    let _our_id: u8 = ID.unwrap_or("0").parse().unwrap();

    defmt::debug!("Launching basestation");

    let mut chip = dwm1001::DWM1001::take().unwrap();

    let mut delay = Delay::new(chip.SYST);
    let mut rng = Rng::new(chip.RNG);

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

    // These are the hardcoded calibration values from the dwm1001-examples
    // repository[1]. Ideally, the calibration values would be determined using
    // the proper calibration procedure, but hopefully those are good enough for
    // now.
    //
    // [1] https://github.com/Decawave/dwm1001-examples
    radio
        .set_antenna_delay(16456, 16300)
        .expect("Failed to set antenna delay");



    let bus = shared_bus::BusManagerSimple::new(chip.LIS2DH12);

    let mut accelerometer = {
        let mut accelerometer =
            Lis2dh12::new(bus.acquire_i2c(), lis2dh12::SlaveAddr::Alternative(true))
                .expect("lis2dh12 new failed");

        accelerometer
            .set_mode(lis2dh12::Mode::HighResolution)
            .expect("lis2dh12 set_mode failed");

        accelerometer
            .set_odr(lis2dh12::Odr::Hz400)
            .expect("lis2dh12 set_odr failed");

        accelerometer
            .enable_axis((true, true, true))
            .expect("lis2dh12 enable_axis failed");

        accelerometer
            .enable_temp(true)
            .expect("lis2dh2 enable_temp failed");

        accelerometer
    };

    let mut mpu = {
        let mut mpu = mpu6050::Mpu6050::new(bus.acquire_i2c());
        mpu.init(&mut delay).unwrap();
        mpu
    };

    loop {
        defmt::info!("Sending ping");

        for _ in 0..3 {
            chip.leds.D10.enable();
            delay.delay_ms(5u32);
            chip.leds.D10.disable();
        }

        let mut readings = [DataReading::default(); 2];

        for reading in readings.iter_mut() {
            *reading = {
                let gyro = mpu.get_gyro().unwrap();
                let accel = accelerometer.accel_raw().unwrap();

                DataReading {
                    gyro_x: gyro[0],
                    gyro_y: gyro[1],
                    gyro_z: gyro[2],

                    accel_x: accel[0],
                    accel_y: accel[1],
                    accel_z: accel[2],
                }
            };
        }

        defmt::info!("Sending data");

        let mut sending = tag::DatalogPacket::new(&mut radio, readings)
            .expect("Failed to initiate request")
            .send(radio, tag::configure_tx())
            .expect("Failed to initiate request transmission");

        defmt::info!("waiting for transmision");

        nb::block!(sending.wait_transmit()).expect("Failed to send data");
        radio = sending.finish_sending().expect("Failed to finish sending");

        defmt::info!("packet sent for transmision");
    }
}
