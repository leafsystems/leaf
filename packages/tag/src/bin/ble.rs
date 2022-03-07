#![no_main]
#![no_std]

use lis2dh12::RawAccelerometer;

use defmt_rtt as _;
use nrf52832_hal::pac::SPIM2;
use panic_probe as _;
use tag::{configure_rx, configure_tx, Msg};

use dwm1001::{
    block_timeout,
    dw1000::{
        mac,
        ranging::{self, Message as _RangingMessage},
        RxConfig,
    },
    nrf52832_hal::{
        gpio::{p0::P0_17, Output, PushPull},
        pac,
        rng::Rng,
        Delay, Spim, Temp, Timer,
    },
    prelude::*,
    DWM1001,
};
use uart_types::DataReading;
use zerocopy::AsBytes;

use rubble::{
    att::{AttUuid, Attribute, AttributeAccessPermissions, AttributeProvider, Handle, HandleRange},
    uuid::{Uuid128, Uuid16},
    Error,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    //
    defmt::info!("Hello, world!");
    loop {
        //
    }
}
