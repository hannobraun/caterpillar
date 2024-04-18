use capi_runtime::{DebugEvent, Expression, LineLocation, Program};
use futures::{
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    future::{select, Either},
    SinkExt, StreamExt,
};
use gloo::net::websocket::{futures::WebSocket, Message};
use leptos::{
    component, create_signal, ev::MouseEvent, view, CollectView, IntoView,
    ReadSignal, SignalGet, SignalSet, WriteSignal,
};
use web_sys::{wasm_bindgen::JsCast, HtmlSpanElement};

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging to console");

    let (code, set_code) = create_signal(Program::default());
    let (events_tx, events_rx) = mpsc::unbounded();

    leptos::spawn_local(handle_server(set_code, events_rx));

    leptos::mount_to_body(
        move || view! { <Debugger code=code events=events_tx /> },
    );

    log::info!("Caterpillar initialized.");
}

#[component]
pub fn Debugger(code: ReadSignal<Program>, events: EventsTx) -> impl IntoView {
    view! {
        {
            move || {
                code.get()
                    .functions
                    .inner
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
    function: capi_runtime::Function,
    events: EventsTx,
) -> impl IntoView {
    let lines = function
        .syntax
        .into_iter()
        .map(|expression| {
            view! {
                <LineWithBreakpoint
                    expression=expression
                    events=events.clone() />
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
pub fn LineWithBreakpoint(
    expression: Expression,
    events: EventsTx,
) -> impl IntoView {
    view! {
        <li class="ml-8">
            <Breakpoint expression=expression.clone() events=events />
            <Line expression=expression />
        </li>
    }
}

#[component]
pub fn Breakpoint(expression: Expression, events: EventsTx) -> impl IntoView {
    let breakpoint_color = if expression.breakpoint {
        "text-red-600"
    } else {
        "text-red-300"
    };

    let class = format!("mr-1 {breakpoint_color}");

    let toggle_breakpoint = move |event: MouseEvent| {
        let event_target = event.target().unwrap();
        let element = event_target.dyn_ref::<HtmlSpanElement>().unwrap();

        let function = element.get_attribute("data-function").unwrap();
        let line: u32 =
            element.get_attribute("data-line").unwrap().parse().unwrap();

        leptos::spawn_local(send_event(
            DebugEvent::ToggleBreakpoint {
                location: LineLocation { function, line },
            },
            events.clone(),
        ));
    };

    // It would be nicer to have the click handler on the enclosing element, to
    // make it less finicky for the user. But for some reason, I'm getting a
    // reference to the window on `event.current_target()`, so I have to rely on
    // `event.target()` to find the metadata. And that means, I can't have
    // events coming from multiple elements.
    //
    // There are probably better ways to solve this problem, but for now, this
    // is fine, if unfortunate.
    view! {
        <span
            class=class
            data-function=expression.location.function
            data-line=expression.location.line
            on:click=toggle_breakpoint>
            {'⦿'}
        </span>
    }
}

#[component]
pub fn Line(expression: Expression) -> impl IntoView {
    let line = format!("{}", expression.kind);

    view! {
        <span>{line}</span>
    }
}

async fn send_event(event: DebugEvent, mut events: EventsTx) {
    if let Err(err) = events.send(event).await {
        log::error!("Error sending event: {err}");
    }
}

async fn handle_server(set_code: WriteSignal<Program>, mut events: EventsRx) {
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

                let program: Program = match msg {
                    Message::Text(text) => serde_json::from_str(&text).unwrap(),
                    Message::Bytes(bytes) => {
                        serde_json::from_slice(&bytes).unwrap()
                    }
                };

                set_code.set(program);
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
