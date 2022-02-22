#![no_main]
#![no_std]

use lis2dh12::{Lis2dh12, RawAccelerometer};

use dwm1001::{
    nrf52832_hal::{rng::Rng, Delay},
    prelude::*,
};
use nrf52832_hal::{
    gpio::p0::{self},
    Timer, Twim,
};

// use rubble::{
//     config::Config,
//     l2cap::{BleChannelMap, L2CAPState},
//     link::{
//         ad_structure::AdStructure,
//         queue::{PacketQueue, SimpleQueue},
//         LinkLayer, Responder, MIN_PDU_BUF,
//     },
//     security::NoSecurity,
//     time::{Duration, Timer as RubbleTimer},
// };
// use rubble_nrf5x::{
//     radio::{BleRadio, PacketBuffer},
//     timer::BleTimer,
//     utils::get_device_address,
// };

use defmt_rtt as _;
use panic_probe as _;

use nrf52832_hal::pac;
use nrf52832_hal::twim;
use zerocopy::AsBytes;

const DATA_BUF_SIZE: usize = 256;
// const DATA_BUF_SIZE: usize = 512;

// static mut ble_tx_buf: PacketBuffer = [0; 39];
// static mut ble_rx_buf: PacketBuffer = [0; 39];

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
            .set_odr(lis2dh12::Odr::Hz1)
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
                loop {
                    dwm.leds.D10.enable();
                    dwm.leds.D11.enable();
                    dwm.leds.D12.enable();
                    delay.delay_ms(50u32);

                    dwm.leds.D10.disable();
                    dwm.leds.D11.disable();
                    dwm.leds.D12.disable();
                    delay.delay_ms(50u32);

                    let mut out: [u8; 6] = [
                        0x70, // P
                        0x69, // I
                        0x6e, // N
                        0x67, // G
                        0x0d, // CR
                        0x0a, // LF
                    ];

                    // wait until we get a ping
                    if dwm.uart.read_timeout(&mut out, &mut timer, 100).is_ok() {
                        // write our buf to the wire
                        dwm.uart.write(readings.as_bytes()).unwrap();
                    }
                }
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

#[repr(packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, zerocopy::AsBytes)]
struct DataReading {
    gyro_x: f32,
    gyro_y: f32,
    gyro_z: f32,
    accel_x: i16,
    accel_y: i16,
    accel_z: i16,
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

// let mut tx_queue = SimpleQueue::new();
// let mut rx_queue = SimpleQueue::new();

// let mut ble = {
//     let (tx, tx_cons) = tx_queue.split();
//     let (rx_prod, rx) = rx_queue.split();

//     // let mut radio =
//     //     unsafe { BleRadio::new(dwm.RADIO, &dwm.FICR, &mut ble_tx_buf, &mut ble_rx_buf) };

//     // // Create the actual BLE stack objects
//     // let mut ble_ll = LinkLayer::<AppConfig>::new(device_address, ble_timer);

//     // radio
// };
