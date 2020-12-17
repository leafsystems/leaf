mod logging;

use anyhow::{Error, Result};
use log::{debug, error, info, warn};
use rppal::uart::{Parity, Uart};
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<()> {
    logging::set_up_logging();

    info!("Starting gateway service");

    // Connect to the primary UART and configure it for 115.2 kbit/s, no
    // parity bit, 8 data bits and 1 stop bit.
    let mut uart = Uart::new(115_200, Parity::None, 8, 1)?;

    // Configure read() to block until at least 1 byte is received.
    uart.set_read_mode(1, Duration::from_secs(0))?;
    // uart.set_read_mode(1, Duration::default())?;

    let mut buffer = [0u8; 1];
    let mut message = Vec::new();

    info!("Buffer assembled");
    loop {
        // TODO! assemble this into a packet
        info!("Reading...");

        // Fill the buffer variable with any incoming data.
        if uart.read(&mut buffer)? > 0 {
            info!("Received byte: {}", buffer[0]);
            info!("Byts are {:?}", buffer);
            message.push(buffer[0]);
            // let text = String::from_utf8(message.clone())?;
            // info!("Message is {}", text);
            reqwest::get("http://192.168.1.24:9001/ring").await?;
        } else {
            info!("uhhhh nothing recived");
        }
    }

    Ok(())
}

// rm gateway
// curl http://192.168.1.24:8000/target/arm-unknown-linux-musleabi/release/gateway --output gateway
// chmod +x gateway
// ./gateway
