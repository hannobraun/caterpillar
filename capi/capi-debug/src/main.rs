use capi_runtime::{DebugState, DebugSyntaxElement};
use futures::{
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    future::{select, Either},
    SinkExt, StreamExt,
};
use gloo::net::websocket::{futures::WebSocket, Message};
use leptos::{
    component, create_signal, view, CollectView, IntoView, ReadSignal,
    SignalGet, SignalSet, WriteSignal,
};

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging to console");

    let (code, set_code) = create_signal(DebugState::default());
    let (events_tx, events_rx) = mpsc::unbounded();

    leptos::spawn_local(fetch_code(set_code, events_rx));

    leptos::mount_to_body(
        move || view! { <Debugger code=code events=events_tx /> },
    );

    log::info!("Capi Debug initialized.");
}

#[component]
pub fn Debugger(
    code: ReadSignal<DebugState>,
    events: UnboundedSender<()>,
) -> impl IntoView {
    view! {
        {
            move || {
                code.get()
                    .functions
                    .into_iter()
                    .map(|f| view! {
                        <Function function=f events=events.clone() />
                    })
                    .collect_view()
            }
        }
    }
}

#[component]
pub fn Function(
    function: capi_runtime::DebugFunction,
    events: UnboundedSender<()>,
) -> impl IntoView {
    let lines = function
        .syntax
        .into_iter()
        .map(|syntax_element| {
            view! {
                <Line syntax_element=syntax_element events=events.clone() />
            }
        })
        .collect_view();

    view! {
        <div class="m-2 mb-4">
            <div class="font-bold">
                {function.name}:{'\n'}
            </div>
            <ol>
                {lines}
            </ol>
        </div>
    }
}

#[component]
pub fn Line(
    syntax_element: DebugSyntaxElement,
    events: UnboundedSender<()>,
) -> impl IntoView {
    let breakpoint_color = if syntax_element.breakpoint {
        "text-red-600"
    } else {
        "text-red-300"
    };
    let line = format!("{}", syntax_element.inner);

    let class = format!("mr-1 {breakpoint_color}");

    let toggle_breakpoint = move |_| {
        leptos::spawn_local(send_event(events.clone()));
    };

    view! {
        <li class="ml-8">
            <span class=class on:click=toggle_breakpoint>{'⦿'}</span>
            <span>{line}</span>
        </li>
    }
}

async fn send_event(mut events: UnboundedSender<()>) {
    if let Err(err) = events.send(()).await {
        log::error!("Error sending event: {err}");
    }
}

async fn fetch_code(
    set_code: WriteSignal<DebugState>,
    mut events: UnboundedReceiver<()>,
) {
    let mut socket = WebSocket::open("ws://127.0.0.1:8080/code").unwrap();

    let mut message = socket.next();
    let mut event = events.next();

    loop {
        match select(message, event).await {
            Either::Left((msg, evt)) => {
                message = socket.next();
                event = evt;

                let Some(msg) = msg else { break };

                let msg = match msg {
                    Ok(msg) => msg,
                    Err(err) => {
                        log::error!("Error receiving WebSocket message: {err}");
                        return;
                    }
                };

                let code: DebugState = match msg {
                    Message::Text(text) => serde_json::from_str(&text).unwrap(),
                    Message::Bytes(bytes) => {
                        serde_json::from_slice(&bytes).unwrap()
                    }
                };

                set_code.set(code);
            }
            Either::Right((evt, msg)) => {
                event = events.next();
                message = msg;

                let Some(_evt) = evt else {
                    log::error!("No more events.");
                    return;
                };

                log::info!("Breakpoint toggled!");
            }
        }
    }
}
