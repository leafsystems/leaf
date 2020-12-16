#![no_std]

// use nrf52832_hal as _; // memory layout
// use nrf52840_hal as _; // memory layout

pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
