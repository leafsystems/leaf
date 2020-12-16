use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
// use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::spawn_local;
use yew::{
    events::MouseEvent, html, App, Callback, Children, Component, ComponentLink, Html, Properties,
    ShouldRender,
};
use yew_functional::{
    use_effect_with_deps, use_reducer_with_init, use_ref, use_state, FunctionComponent,
    FunctionProvider,
};

pub fn CB<T, F: Fn(T) + 'static>(infn: F) -> Callback<T> {
    let newfn = Rc::new(infn);
    Callback::Callback(newfn)
}

use std::future::Future;

pub fn handle_future<F, H, T>(future: F, handler: H)
where
    F: Future<Output = T> + 'static,
    H: Fn(T) -> () + 'static,
{
    spawn_local(async move {
        let rs: T = future.await;
        handler(rs);
    });
}
