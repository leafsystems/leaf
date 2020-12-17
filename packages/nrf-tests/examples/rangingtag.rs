// #![no_std]
// #![no_main]

// use cortex_m_rt::entry;
// use defmt_rtt as _;

// use dwm1001::{
//     block_timeout, debug,
//     dw1000::{
//         mac,
//         ranging::{self, Message as _RangingMessage},
//         RxConfig,
//     },
//     nrf52832_hal::{
//         gpio::{p0::P0_17, Output, PushPull},
//         pac::SPIM2,
//         rng::Rng,
//         Delay, Spim, Timer,
//     },
//     prelude::*,
//     print, DWM1001,
// };

// #[entry]
// fn main() -> ! {
//     let mut dwm1001 = DWM1001::take().unwrap();

//     let mut delay = Delay::new(dwm1001.SYST);
//     let mut rng = Rng::new(dwm1001.RNG);

//     dwm1001.DW_RST.reset_dw1000(&mut delay);
//     let mut dw1000 = dwm1001.DW1000.init().expect("Failed to initialize DW1000");

//     dw1000
//         .enable_tx_interrupts()
//         .expect("Failed to enable TX interrupts");
//     dw1000
//         .enable_rx_interrupts()
//         .expect("Failed to enable RX interrupts");

//     let mut dw_irq = dwm1001.DW_IRQ;
//     let mut gpiote = dwm1001.GPIOTE;

//     // These are the hardcoded calibration values from the dwm1001-examples
//     // repository[1]. Ideally, the calibration values would be determined using
//     // the proper calibration procedure, but hopefully those are good enough for
//     // now.
//     //
//     // [1] https://github.com/Decawave/dwm1001-examples
//     dw1000
//         .set_antenna_delay(16456, 16300)
//         .expect("Failed to set antenna delay");

//     // Set network address
//     dw1000
//         .set_address(
//             mac::PanId(0x0d57),                  // hardcoded network id
//             mac::ShortAddress(rng.random_u16()), // random device address
//         )
//         .expect("Failed to set address");

//     let mut timeout_timer = Timer::new(dwm1001.TIMER1);

//     let mut buf = [0; 128];

//     loop {
//         defmt::info!("Entering receve mode...");
//         let mut receiving = dw1000
//             .receive(RxConfig::default())
//             .expect("Failed to receive message");

//         timeout_timer.start(500_000u32);
//         let message = block_timeout!(&mut timeout_timer, {
//             // dw_irq.wait_for_interrupts(&mut gpiote, &mut timeout_timer);
//             receiving.wait(&mut buf)
//         });

//         defmt::info!("Timeout up !");

//         dw1000 = receiving
//             .finish_receiving()
//             .expect("Failed to finish receiving");
//         defmt::info!("finished receving !");

//         let message = match message {
//             Ok(message) => message,
//             Err(_) => {
//                 defmt::info!("no message :(");
//                 continue;
//             } //ignore error
//         };

//         let ping = ranging::Ping::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>(&message)
//             .expect("Failed to decode ping");

//         defmt::info!("Message received!");
//         if let Some(ping) = ping {
//             // Received ping from an anchor. Reply with a ranging
//             // request.
//             dwm1001.leds.dwm1001.leds.D9.enable();
//             delay.delay_ms(10u32);
//             dwm1001.leds.D9.disable();

//             // Wait for a moment, to give the anchor a chance to start listening
//             // for the reply.
//             delay.delay_ms(10u32);

//             let mut sending = ranging::Request::new(&mut dw1000, &ping)
//                 .expect("Failed to initiate request")
//                 .send(dw1000)
//                 .expect("Failed to initiate request transmission");

//             timeout_timer.start(500_000u32);
//             block_timeout!(&mut timeout_timer, {
//                 dw_irq.wait_for_interrupts(&mut gpiote, &mut timeout_timer);
//                 sending.wait()
//             })
//             .expect("Failed to send ranging request");

//             dw1000 = sending.finish_sending().expect("Failed to finish sending");

//             continue;
//         }

//         let response = ranging::Response::decode::<Spim<SPIM2>, P0_17<Output<PushPull>>>(&message)
//             .expect("Failed to decode response");
//         if let Some(response) = response {
//             // Received ranging response from anchor. Now we can compute the
//             // distance.

//             dwm1001.leds.D9.enable();
//             delay.delay_ms(10u32);
//             dwm1001.leds.D9.disable();

//             // If this is not a PAN ID and short address, it doesn't
//             // come from a compatible node. Ignore it.
//             let (pan_id, addr) = match response.source {
//                 mac::Address::Short(pan_id, addr) => (pan_id, addr),
//                 _ => continue,
//             };

//             // Ranging response received. Compute distance.
//             let distance_mm = ranging::compute_distance_mm(&response).unwrap();

//             dwm1001.leds.D9.enable();
//             delay.delay_ms(10u32);
//             dwm1001.leds.D9.disable();

//             defmt::info!("Distance measured... {:?}", distance_mm);
//             // print!("{:04x}:{:04x} - {} mm\n", pan_id.0, addr.0, distance_mm,);
//             // print!("{:04x}:{:04x} - {} mm\n", pan_id.0, addr.0, distance_mm,);

//             continue;
//         }

//         defmt::info!("ignored message");
//         print!("Ignored message that was neither ping nor response\n");
//     }
// }

// #[panic_handler]
// pub fn panic(_info: &core::panic::PanicInfo) -> ! {
//     defmt::error!("panicked");
//     exit()
// }

// pub fn exit() -> ! {
//     loop {
//         cortex_m::asm::bkpt();
//     }
// }