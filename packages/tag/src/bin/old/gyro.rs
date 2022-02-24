#![no_main]
#![no_std]




use defmt_rtt as _;


use nrf52832_hal::{
    gpio::p0::{self},
    Twim,
};
use panic_probe as _;

use dwm1001::{
    nrf52832_hal::{
        Delay,
    },
    prelude::*,
};

use nrf52832_hal::pac;
use nrf52832_hal::twim;


#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    let mut delay = Delay::new(cp.SYST);
    let port0 = p0::Parts::new(p.P0);

    let i2c = new_acc_twim(p.TWIM1, port0.p0_28, port0.p0_29);

    let mut mpu = mpu6050::Mpu6050::new(i2c);
    mpu.init(&mut delay).unwrap();

    let addr = mpu.read_byte(mpu6050::device::WHOAMI).unwrap();

    defmt::info!("{:?}", addr);

    loop {
        let acc = mpu.get_gyro().unwrap();

        defmt::info!("{:?}", acc.as_slice());

        delay.delay_ms(50u32);
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
