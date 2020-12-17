use std::fmt::Display;

use common_interfaces::GatewayEvent;
use wasm_bindgen::JsCast;
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::MessageEvent;
use yew::{services::websocket::WebSocketTask, Callback};

use yew_functional::{use_effect_with_deps, use_hook, Hook, HookUpdater};
use yew_recoil::{
    prelude::use_recoil_state,
    readable::{atom, Atom},
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SensorData {}

pub static SENSOR: Atom<SensorData> = atom(|_| {});

pub fn use_websocket<D: 'static>(addr: &'static str, cb: impl FnOnce(D) -> () + 'static)
where
    D: for<'a> serde::de::Deserialize<'a> + std::fmt::Debug,
{
    use_hook::<WebsocketHook<_, _>, _>(cb, move || {
        let task = web_sys::WebSocket::new(addr).expect("failed to make websocket");

        WebsocketHook {
            task,
            _out: std::marker::PhantomData::default(),
            _out2: std::marker::PhantomData::default(),
        }
    });
}

struct WebsocketHook<D, F> {
    task: web_sys::WebSocket,
    _out: std::marker::PhantomData<D>,
    _out2: std::marker::PhantomData<F>,
}

impl<D, F: FnOnce(D) -> () + 'static> yew_functional::Hook for WebsocketHook<D, F>
where
    D: for<'a> serde::de::Deserialize<'a> + std::fmt::Debug,
{
    type Output = ();
    type Args = F;
    fn runner(&mut self, args: Self::Args, updater: HookUpdater) -> Self::Output {
        let onmessage_callback = Closure::once(Box::new(move |e: MessageEvent| {
            let data: JsValue = e.data();
            log::info!("data is {:#?}", data);
            let converted = data.into_serde::<String>().unwrap();
            let converted: D = serde_json::from_str(converted.as_str()).unwrap();
            // let converted = data.into_serde::<D>().unwrap();
            log::info!("converted is {:#?}", converted);
            // // let converted: D = data.into_serde().unwrap();
            args(converted);
        }) as Box<dyn FnOnce(MessageEvent)>);
        // }) as Box<dyn FnMut(MessageEvent)>);

        // set message event handler on WebSocket
        self.task
            .set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        // forget the callback to keep it alive
        onmessage_callback.forget();

        // todo!()
    }
}
