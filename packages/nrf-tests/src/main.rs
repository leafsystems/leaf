#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
// use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");

    loop {
        asm::bkpt()
    }
}
