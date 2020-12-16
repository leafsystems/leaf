use yew_functional::{use_effect_with_deps, Hook, HookUpdater};
use yew_recoil::{
    prelude::use_recoil_state,
    readable::{atom, Atom},
};

#[derive(Default, Debug, Clone, PartialEq)]
struct SensorData {}

pub static SENSOR: Atom<SensorData> = atom(|_| {});

pub fn use_websocket() {
    let g = String::from("asdas");

    let (_, set_data) = use_recoil_state(&SENSOR);

    let effect = move |ev: &_| {
        log::info!("hello world!");
        log::info!("hello {}!", g);

        || {}
    };

    use_effect_with_deps(effect, ());
}
