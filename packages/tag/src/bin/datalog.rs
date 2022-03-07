#![no_main]
#![no_std]

use lis2dh12::{Lis2dh12, RawAccelerometer};

use defmt_rtt as _;
use dwm1001::{
    nrf52832_hal::{rng::Rng, Delay},
    prelude::*,
};
use nrf52832_hal::{
    gpio::p0::{self},
    pac, twim, Timer, Twim,
};
use panic_probe as _;

use uart_types::{DataReading, DATA_BUF_SIZE};
use zerocopy::AsBytes;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::debug!("Launching basestation");

    let mut dwm = dwm1001::DWM1001::take().unwrap();
    let mut delay = Delay::new(dwm.SYST);
    let _rng = Rng::new(dwm.RNG);

    let bus = shared_bus::BusManagerSimple::new(dwm.LIS2DH12);

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

    let mut timer = Timer::new(dwm.TIMER0);

    let mut readings = [DataReading::default(); DATA_BUF_SIZE];
    let mut cur_reading = 0;

    loop {
        match cur_reading {
            0 => {
                for _x in 0..5 {
                    dwm.leds.D10.enable();
                    dwm.leds.D11.enable();
                    dwm.leds.D12.enable();
                    delay.delay_ms(50u32);

                    dwm.leds.D10.disable();
                    dwm.leds.D11.disable();
                    dwm.leds.D12.disable();
                    delay.delay_ms(50u32);
                }
                cur_reading += 1;
            }

            DATA_BUF_SIZE => {
                // poll the uart chanel until we get a packet
                dwm.leds.D10.enable();
                dwm.leds.D11.enable();
                dwm.leds.D12.enable();
                delay.delay_ms(50u32);

                dwm.leds.D10.disable();
                dwm.leds.D11.disable();
                dwm.leds.D12.disable();
                delay.delay_ms(50u32);

                let mut ping: [u8; 6] = [
                    0x00, // P
                    0x00, // I
                    0x00, // N
                    0x00, // G
                    0x00, // CR
                    0x00, // LF
                          // 0x70, // P
                          // 0x69, // I
                          // 0x6e, // N
                          // 0x67, // G
                          // 0x0d, // CR
                          // 0x0a, // LF
                ];

                defmt::info!("waiting for log ping");

                dwm.uart.read(&mut ping).unwrap();

                // let as_bytes = readings.as_bytes();

                // for reading in readings.iter() {

                // dwm.uart.write(readings.as_bytes()).unwrap();
                // }

                // delay.delay_ms(2000u32);

                defmt::info!("readings ready to send");

                for reading in readings.iter() {
                    dwm.uart.write(reading.as_bytes()).unwrap();
                }

                defmt::info!("readings sent!");

                cur_reading = 0;

                // // wait until we get a ping
                // if dwm.uart.read_timeout(&mut out, &mut timer, 100).is_ok() {
                //     // write our buf to the wire
                // }
            }

            _ => {
                let reading = {
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

                readings[cur_reading] = reading;

                delay.delay_ms(15u32);

                cur_reading += 1;
            }
        }
    }
}

/// Create a new instance of the TWIM bus used for the accelerometer
pub fn new_acc_twim<SCL, SDA>(
    twim: pac::TWIM1,
    scl: p0::P0_28<SCL>,
    sda: p0::P0_29<SDA>,
) -> Twim<pac::TWIM1> {
    Twim::new(
        twim,
        twim::Pins {
            scl: scl.into_floating_input().degrade(),
            sda: sda.into_floating_input().degrade(),
        },
        twim::Frequency::K100,
    )
}
