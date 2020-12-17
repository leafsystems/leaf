#![no_std]

mod board;
mod error;
pub use board::*;

// #[panic_handler] // panicking behavior
// pub fn panic(_: &core::panic::PanicInfo) -> ! {
//     loop {
//         cortex_m::asm::bkpt();
//     }
// }

// struct RTOS {}

// trait RtosApp {}

// struct App {}

// impl RtosApp for App {}

// fn start() {
//     RTOS::<App>::new()
//         .interrupt(|| {})
//         .event(|| { /*event source*/ }, |evt| { /* code */ })
//         .task(PriorityLevel::Low, || {})
//         .task(|board, ctx| async {
//             // broadcast ssid
//         })
//         .task(|board, ctx| async {
//             // reply to any messages periodically
//         })
//         .task(|board, ctx| async {})
//         .task(|board, ctx| async {})
//         .launch();
// }
