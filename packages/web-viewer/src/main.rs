// mod hooks;
mod api;
mod app;
mod util;

// Called when the wasm module is instantiated
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::MyApp>();
}
