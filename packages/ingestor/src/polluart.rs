use std::time::Duration;

use futures_channel::mpsc::UnboundedSender;

use serialport::Parity;

use crate::{Msg, UartUpdate};

pub fn poll_uart(sender: UnboundedSender<UartUpdate>) {
    let info = serialport::available_ports()
        .unwrap()
        .into_iter()
        .find(|f| f.port_name.starts_with("/dev/tty.usbmodem"))
        .unwrap();

    let mut port = serialport::new(info.port_name, 115200)
        .parity(Parity::None)
        .timeout(Duration::from_millis(5000))
        .open()
        .unwrap();

    let mut idx = 0;
    let mut buf = [0u8; 24];
    loop {
        // this sorta suffers from not
        match port.read_exact(&mut buf) {
            Ok(_num_read) => {
                let msg: Msg = postcard::from_bytes(&buf).unwrap();

                sender.unbounded_send(UartUpdate::Ranging(msg)).unwrap();

                idx += 1;
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
