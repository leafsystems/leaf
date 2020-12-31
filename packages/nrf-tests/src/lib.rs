#![no_std]

use core::future::Future;

use defmt_rtt as _;
use dwm1001::nrf52832_hal as _;
use panic_probe as _;

pub struct Runtime {}

impl Runtime {
    fn initialize<T>(f: fn() -> T) -> Self {
        todo!()
    }

    fn with_task<Fut, O>(self, f: fn() -> Fut) -> Self
    where
        Fut: Future<Output = O>,
    {
        self
    }

    fn on_progress<Fut, O>(self, f: fn() -> Fut) -> Self
    where
        Fut: Future<Output = O>,
    {
        self
    }

    fn on_trigger<Fut, O>(self, f: fn() -> Fut) -> Self
    where
        Fut: Future<Output = O>,
    {
        self
    }

    fn launch(mut self) -> ! {
        todo!()
    }
}

struct Task<'a> {
    g: &'a mut Runtime,
}

mod tests {
    use super::*;

    struct MyBoard {}

    /*
    Thesis:

    Adding new functionality to a microcontroller adds controlflow overhead.
    If modeled as a state machine, this causes serious changes to how data is shared
    throughout the program, and which tasks get priority over others.

    There are many different types of programming paradigms, patterns, and structures that
    attempt to work with this event-based model. In many ways, a microcontroller is very
    similar to a webpage or a webserver. External events and internal timers drive
    the progress and evolution of the page's state.

    For a react component, the resulting viewed state is a function of the inputs and some
    internal state, stored by hooks or directly on the component. Updates are triggered by




    */

    pub fn test_launch() -> ! {
        // Goal is to make it feel like writing web-server applications
        // Multithreading, io, etc

        Runtime::initialize(|| MyBoard {
            // setup the board here
            // configure pins, timers, etc
            // enable the leds
        })
        .on_progress(|| async {})
        .with_task(|| async {})
        .with_task(|| async {})
        .with_task(|| async {})
        .with_task(|| async {})
        .launch()
    }
}
