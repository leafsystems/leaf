#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

use dwm1001::dw1000::{
    self,
    configs::{BitRate, PreambleLength, SfdSequence},
    RxConfig, TxConfig,
};
use dwm1001::nrf52832_hal::{
    prelude::*,
    timer::{self, Timer},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Msg {
    pub id: u8,
    pub temp: f32,
    pub accel: (i16, i16, i16),
    pub distance: u64,
}

pub fn configure_rx() -> RxConfig {
    RxConfig {
        bitrate: BitRate::Kbps6800,
        channel: dw1000::configs::UwbChannel::Channel1,
        expected_preamble_length: PreambleLength::Symbols64,
        sfd_sequence: SfdSequence::IEEE,
        ..Default::default()
    }
}

pub fn configure_tx() -> TxConfig {
    TxConfig {
        bitrate: BitRate::Kbps6800,
        channel: dw1000::configs::UwbChannel::Channel1,
        preamble_length: PreambleLength::Symbols64,
        sfd_sequence: SfdSequence::IEEE,
        ..Default::default()
    }
}

pub fn delay_timer<T>(timer: &mut Timer<T>, cycles: u32)
where
    T: timer::Instance,
{
    timer.start(cycles);
    nb::block!(timer.wait()).unwrap();
}
