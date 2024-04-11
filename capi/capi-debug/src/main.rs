use capi_runtime::{DebugEvent, DebugState, DebugSyntaxElement};
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

    leptos::spawn_local(handle_server(set_code, events_rx));

    leptos::mount_to_body(
        move || view! { <Debugger code=code events=events_tx /> },
    );

    log::info!("Capi Debug initialized.");
}

#[component]
pub fn Debugger(
    code: ReadSignal<DebugState>,
    events: EventsTx,
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
    events: EventsTx,
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
    events: EventsTx,
) -> impl IntoView {
    let breakpoint_color = if syntax_element.breakpoint {
        "text-red-600"
    } else {
        "text-red-300"
    };
    let line = format!("{}", syntax_element.inner);

    let class = format!("mr-1 {breakpoint_color}");

    let toggle_breakpoint = move |_| {
        leptos::spawn_local(send_event(
            DebugEvent::ToggleBreakpoint,
            events.clone(),
        ));
    };

    view! {
        <li class="ml-8">
            <span class=class on:click=toggle_breakpoint>{'⦿'}</span>
            <span>{line}</span>
        </li>
    }
}

async fn send_event(event: DebugEvent, mut events: EventsTx) {
    if let Err(err) = events.send(event).await {
        log::error!("Error sending event: {err}");
    }
}

async fn handle_server(
    set_code: WriteSignal<DebugState>,
    mut events: EventsRx,
) {
    let mut socket = WebSocket::open("ws://127.0.0.1:8080/code").unwrap();

    loop {
        // When one future gets selected, we drop the other one. I assume that
        // this doesn't consume an item in the stream then.
        //
        // I had another implementation here before, that kept around all the
        // futures until they're polled to completion. That would prevent
        // sending anything through the socket though (as the existing future
        // borrows the socket).
        //
        // Let's hope this works out! If weird stuff starts happening, this
        // might be one of the places to take a closer look at.
        match select(socket.next(), events.next()).await {
            Either::Left((msg, _)) => {
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
            Either::Right((evt, _)) => {
                let Some(evt) = evt else {
                    log::error!("No more events.");
                    return;
                };

                let evt = serde_json::to_string(&evt).unwrap();
                socket.send(Message::Text(evt)).await.unwrap();
            }
        }
    }
}

pub type EventsTx = UnboundedSender<DebugEvent>;
pub type EventsRx = UnboundedReceiver<DebugEvent>;
