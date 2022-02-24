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

#[cortex_m_rt::entry]
fn main() -> ! {
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
        .set_antenna_delay(16456, 16300)
        .expect("Failed to set antenna delay");

    // Set network address
    radio
        .set_address(
            mac::PanId(0x0d57),                  // hardcoded network id
            mac::ShortAddress(rng.random_u16()), // random device address
        )
        .expect("Failed to set address");

    let mut timer = Timer::new(chip.TIMER0);
    let mut uart_buf = [0u8; 1024];

    loop {
        flash_lights(&mut chip.leds, &mut timer, 5_000, 5_000);

        defmt::debug!("waiting for base mobile tag ping");

        let mut receiving = radio
            .receive(configure_rx())
            .expect("Failed to receive message");

        let message = nb::block!(receiving.wait_receive(&mut uart_buf));

        radio = receiving
            .finish_receiving()
            .expect("Failed to finish receiving");

        let message = match message {
            Ok(message) => message,
            Err(er) => {
                defmt::error!("Timeout error occured {:?}", defmt::Debug2Format(&er));
                continue;
            }
        };

        let data =
            match tag::DatalogPacket::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>(&message) {
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
        let ix = data.payload.data[0];
        defmt::info!(
            "Received data. {:?} {:?} {:?}",
            ix.gyro_x,
            ix.gyro_y,
            ix.gyro_z
        );
    }
}

fn flash_lights(
    leds: &mut dwm1001::Leds,
    timer: &mut Timer<nrf52832_hal::pac::TIMER0>,
    on: u32,
    off: u32,
) {
    leds.D10.enable();
    tag::delay_timer(timer, on); // 20ms
    leds.D10.disable();
    tag::delay_timer(timer, off); // 230ms
}
