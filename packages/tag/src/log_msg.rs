use core::mem::size_of;

use dwm1001::dw1000::ranging::{Message, Prelude, TxMessage};
use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use serde::{Deserialize, Serialize};
use ssmarshal;

use dwm1001::dw1000::configs::{PulseRepetitionFrequency, UwbChannel};
use dwm1001::dw1000::hl::SendTime;
use dwm1001::dw1000::{
    hl, mac,
    time::{Duration, Instant},
    Error, Ready, Sending, TxConfig, DW1000,
};
use uart_types::DataReading;

const TX_DELAY: u32 = 10_000_000;

#[derive(Debug, Deserialize, Serialize)]
#[repr(C)]
pub struct DatalogPacket {
    pub data: [DataReading; 2],
    // /// When the original ping was sent, in local time on the anchor
    // pub ping_tx_time: Instant,

    // /// The time between the ping being received and the reply being sent
    // pub ping_reply_time: Duration,

    // /// When the ranging request was sent, in local sender time
    // pub request_tx_time: Instant
}

impl Message for DatalogPacket {
    const PRELUDE: Prelude = Prelude(b"RANGING PING");
    const PRELUDE_LEN: usize = 12;
    // const PRELUDE: Prelude = Prelude(b"DATALOG");
    // const PRELUDE_LEN: usize = 7;
}

impl DatalogPacket {
    /// Creates a new ranging request message
    ///
    /// Only creates the message, but doesn't yet send it. Sets the transmission
    /// time to 10 milliseconds in the future. Make sure to send the message
    /// within that time frame, or the distance measurement will be negatively
    /// affected.
    pub fn new<SPI, CS>(
        dw1000: &mut DW1000<SPI, CS, Ready>,
        data: [DataReading; 2],
    ) -> Result<TxMessage<Self>, Error<SPI, CS>>
    where
        SPI: spi::Transfer<u8> + spi::Write<u8>,
        CS: OutputPin,
    {
        let tx_time = dw1000.sys_time()? + Duration::from_nanos(TX_DELAY);

        // let request_tx_time = tx_time + dw1000.get_tx_antenna_delay()?;

        // let ping_reply_time = request_tx_time.duration_since(ping.rx_time);

        // let payload = Request {
        //     ping_tx_time: ping.payload.ping_tx_time,
        //     ping_reply_time,
        //     request_tx_time,
        // };

        Ok(TxMessage {
            recipient: mac::Address::broadcast(&mac::AddressMode::Short),
            tx_time,
            payload: DatalogPacket { data },
        })
    }
}
