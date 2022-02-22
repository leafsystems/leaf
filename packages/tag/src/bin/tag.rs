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
        defmt::info!("waiting for uart comands");

        dwm.uart.read(&mut uart_buf).unwrap();
    }
}
