use futures::lock::Mutex;
mod polluart;
mod ui;

fn main() {
    let (sender, receiver) = futures_channel::mpsc::unbounded();
    std::thread::spawn(move || polluart::poll_uart(sender));

    dioxus::desktop::launch_with_props(
        ui::app,
        ui::AppProps {
            receiver: std::sync::Arc::new(Mutex::new(receiver)),
        },
        |c| c,
    );
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub struct Msg {
    pub id: u8,
    pub temp: f32,
    pub accel: (i16, i16, i16),
    pub distance: u64,
}

#[derive(Clone, Debug, Copy)]
pub enum UartUpdate {
    Ranging(Msg),
}
