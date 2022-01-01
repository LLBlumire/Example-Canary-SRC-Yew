use canary::ServiceAddr;
use srpc::IntoClient;
use yew::prelude::*;
use server::{Pong, PingServer, Ping};
use wasm_bindgen_futures::spawn_local;

#[function_component(App)]
fn app() -> Html {
    let message = use_state(|| None::<Pong>);

    {
        let message = message.clone();
        use_effect_with_deps(move |_| {
            if message.is_none() {
                spawn_local(async move {
                    let addr = "ping://wss@127.0.0.1:8085".parse::<ServiceAddr>().unwrap();
                    let connection = addr.connect().await.unwrap();
                    let mut ping = connection.client::<PingServer>();
                    let pong = ping.ping(Ping(42)).await.unwrap();
                    message.set(Some(pong))
                });
            }
            move || ()
        }, ());
    }

    html! {
        <>
            <h1>{"Page loaded"}</h1>
            {
                if let Some(Pong(pong)) = *message {
                    html! { <h2>{"Received pong with value "}{pong}</h2> }
                } else {
                    html! { <h2>{"Sending ping with value 42..."}</h2> }
                }
            }
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
