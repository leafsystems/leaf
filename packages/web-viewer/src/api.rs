use common_interfaces::GatewayEvent;
use reqwest;
use yew::{html, Callback, Html};
use yew_functional::{use_state, FunctionComponent, FunctionProvider};

pub async fn get_data() -> Vec<GatewayEvent> {
    reqwest::get("http://127.0.0.1:9001/count")
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
