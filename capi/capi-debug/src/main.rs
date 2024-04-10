use capi_runtime::DebugState;
use futures::StreamExt;
use gloo::net::websocket::{futures::WebSocket, Message};
use leptos::{
    component, create_signal, view, CollectView, IntoView, SignalGet,
    SignalSet, WriteSignal,
};

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging to console");

    let (code, set_code) = create_signal(None);
    leptos::spawn_local(fetch_code(set_code));

    let code = move || {
        code.get().map(|code| {
            code.functions
                .into_iter()
                .map(|f| view! { <Function f=f/> })
                .collect_view()
        })
    };

    leptos::mount_to_body(move || code);

    log::info!("Capi Debug initialized.");
}

#[component]
pub fn Function(f: capi_runtime::Function) -> impl IntoView {
    let lines = f
        .syntax
        .into_iter()
        .map(|syntax| {
            let line = format!("{syntax}");
            view! { <li class="ml-8">{line}</li> }
        })
        .collect_view();

    view! {
        <div class="m-2 mb-4">
            <div class="font-bold">
                {f.name}:{'\n'}
            </div>
            <ol>
                {lines}
            </ol>
        </div>
    }
}

async fn fetch_code(set_code: WriteSignal<Option<DebugState>>) {
    let mut socket = WebSocket::open("ws://127.0.0.1:8080/code").unwrap();

    while let Some(message) = socket.next().await {
        let message = match message {
            Ok(message) => message,
            Err(err) => {
                log::error!("Error receiving WebSocket message: {err}");
                return;
            }
        };

        let code: DebugState = match message {
            Message::Text(text) => serde_json::from_str(&text).unwrap(),
            Message::Bytes(bytes) => serde_json::from_slice(&bytes).unwrap(),
        };

        set_code.set(Some(code));
    }
}
