use capi_runtime::{
    DebugEvent, Expression, ExpressionKind, InstructionAddress, Program,
    ProgramEffect, ProgramState,
};
use futures::{
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    future::{select, Either},
    SinkExt, StreamExt,
};
use gloo::net::websocket::{futures::WebSocket, Message};
use leptos::{
    component, create_memo, create_signal, ev::MouseEvent, view, CollectView,
    IntoView, ReadSignal, SignalGet, SignalSet, WriteSignal,
};
use web_sys::{wasm_bindgen::JsCast, HtmlSpanElement};

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging to console");

    let (program, set_program) = create_signal(None);
    let (events_tx, events_rx) = mpsc::unbounded();

    leptos::spawn_local(handle_server(set_program, events_rx));

    leptos::mount_to_body(
        move || view! { <Debugger program=program events=events_tx /> },
    );

    log::info!("Caterpillar initialized.");
}

#[component]
pub fn Debugger(
    program: ReadSignal<Option<Program>>,
    events: EventsTx,
) -> impl IntoView {
    view! {
        <ProgramState
            program=program />
        <CallStack
            program=program />
        <ExecutionContext
            program=program />
        <DataStack
            program=program />
        <Memory
            program=program />
        <ResetButton
            events=events.clone() />
        <CodeExplorer
            program=program
            events=events />
    }
}

#[component]
pub fn ProgramState(program: ReadSignal<Option<Program>>) -> impl IntoView {
    move || {
        let program = program.get()?;

        Some(view! {
            <p>"Program state: "{move || format!("{:?}", program.state)}</p>
        })
    }
}

#[component]
pub fn CallStack(program: ReadSignal<Option<Program>>) -> impl IntoView {
    let addresses = move || {
        let program = program.get()?;

        let view = program
            .evaluator
            .call_stack
            .into_iter()
            .filter_map(|address| {
                let location =
                    program.source_map.address_to_location(&address)?;

                Some(view! {
                    <li>{format!("{location:?}")}</li>
                })
            })
            .collect_view();

        Some(view)
    };

    view! {
        <div>
            <h2>"Call stack:"</h2>
            <ol>
                {addresses}
            </ol>
        </div>
    }
}

#[component]
pub fn ExecutionContext(program: ReadSignal<Option<Program>>) -> impl IntoView {
    move || {
        let Some(program) = program.get() else {
            return view! {
                <p>"No program available."</p>
            };
        };

        let (_effect, _address) = match program.state {
            ProgramState::Running => {
                return view! {
                    <p>"Program is running."</p>
                }
            }
            ProgramState::Finished => {
                return view! {
                    <p>"Program has finished running."</p>
                }
            }
            ProgramState::Effect { effect, address } => (effect, address),
        };

        view! {
            <p>"Placeholder for execution context"</p>
        }
    }
}

#[allow(unused_braces)] // working around a warning from the `view!` macro
#[component]
pub fn DataStack(program: ReadSignal<Option<Program>>) -> impl IntoView {
    let data_stack = move || {
        let program = program.get()?;

        let view = view! {
            <div>
                <p>
                    "Previous data stack: "
                    {format!("{:?}", program.previous_data_stack)}
                </p>
                <p>
                    "Current data stack: "
                    {format!("{:?}", program.evaluator.data_stack)}
                </p>
            </div>
        };

        Some(view)
    };

    view! {
        {data_stack}
    }
}

#[allow(unused_braces)] // working around a warning from the `view!` macro
#[component]
pub fn Memory(program: ReadSignal<Option<Program>>) -> impl IntoView {
    let memory = move || {
        let program = program.get()?;

        let view = view! {
            <div>
                <p>
                    "Current memory: "
                    {format!("{:?}", program.memory)}
                </p>
            </div>
        };

        Some(view)
    };

    view! {
        {memory}
    }
}

#[component]
pub fn ResetButton(events: EventsTx) -> impl IntoView {
    let send_reset = move |_| {
        leptos::spawn_local(send_event(DebugEvent::Reset, events.clone()));
    };

    view! {
        <input
            type="button"
            value="Reset"
            class="m-1 px-1 bg-gray-300 font-bold"
            on:click=send_reset />
    }
}

#[component]
pub fn CodeExplorer(
    program: ReadSignal<Option<Program>>,
    events: EventsTx,
) -> impl IntoView {
    let functions = move || {
        let view = program
            .get()?
            .functions
            .inner
            .into_iter()
            .map(|f| {
                view! {
                    <Function
                        program=program
                        function=f
                        events=events.clone() />
                }
            })
            .collect_view();

        Some(view)
    };

    view! {
        <div>{functions}</div>
    }
}

#[component]
pub fn Function(
    program: ReadSignal<Option<Program>>,
    function: capi_runtime::Function,
    events: EventsTx,
) -> impl IntoView {
    let lines = function
        .syntax
        .into_iter()
        .map(|expression| {
            view! {
                <LineWithBreakpoint
                    program=program
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
    program: ReadSignal<Option<Program>>,
    expression: Expression,
    events: EventsTx,
) -> impl IntoView {
    let location = expression.location.clone();
    let address = create_memo(move |_| {
        program.get()?.source_map.location_to_address(&location)
    });

    let breakpoint = move || {
        let address = address.get()?;

        Some(view! {
            <Breakpoint
                program=program
                address=address
                events=events.clone() />
        })
    };

    view! {
        <li class="ml-8">
            {breakpoint}
            <Line
                program=program
                expression=expression />
        </li>
    }
}

#[component]
pub fn Breakpoint(
    program: ReadSignal<Option<Program>>,
    address: InstructionAddress,
    events: EventsTx,
) -> impl IntoView {
    let class = move || {
        let program = program.get()?;

        let breakpoint_color = if program.breakpoint_at(&address) {
            "text-green-600"
        } else {
            "text-green-300"
        };

        Some(format!("mr-1 {breakpoint_color}"))
    };

    let data_address = move || address.to_usize();

    let toggle_breakpoint = move |event: MouseEvent| {
        let event_target = event.target().unwrap();
        let element = event_target.dyn_ref::<HtmlSpanElement>().unwrap();

        let address = element
            .get_attribute("data-address")
            .unwrap()
            .parse()
            .unwrap();

        leptos::spawn_local(send_event(
            DebugEvent::ToggleBreakpoint { address },
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
            data-address=data_address
            on:click=toggle_breakpoint>
            {'⦿'}
        </span>
    }
}

#[component]
pub fn Line(
    program: ReadSignal<Option<Program>>,
    expression: Expression,
) -> impl IntoView {
    let is_comment = matches!(expression.kind, ExpressionKind::Comment { .. });
    let class = move || {
        let program = program.get()?;
        let state = program.state;

        let text_classes = if is_comment {
            "italic text-gray-500"
        } else {
            ""
        };

        let bg_class = match state {
            ProgramState::Effect { effect, address }
                if program
                    .source_map
                    .address_to_location(&address)
                    .as_ref()
                    == Some(&expression.location) =>
            {
                match effect {
                    ProgramEffect::Paused => "bg-green-300",
                    _ => "bg-red-300",
                }
            }
            _ => "",
        };

        Some(format!("px-0.5 {text_classes} {bg_class}"))
    };
    let line = format!("{}", expression.kind);

    view! {
        <span class=class>{line}</span>
    }
}

async fn send_event(event: DebugEvent, mut events: EventsTx) {
    if let Err(err) = events.send(event).await {
        log::error!("Error sending event: {err}");
    }
}

async fn handle_server(
    program: WriteSignal<Option<Program>>,
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

                let new_program: Program = match msg {
                    Message::Text(text) => ron::from_str(&text).unwrap(),
                    Message::Bytes(bytes) => {
                        panic!(
                            "Unexpectedly received binary message: {bytes:?}"
                        );
                    }
                };

                program.set(Some(new_program));
            }
            Either::Right((evt, _)) => {
                let Some(evt) = evt else {
                    log::error!("No more events.");
                    return;
                };

                let evt = ron::to_string(&evt).unwrap();
                socket.send(Message::Text(evt)).await.unwrap();
            }
        }
    }
}

pub type EventsTx = UnboundedSender<DebugEvent>;
pub type EventsRx = UnboundedReceiver<DebugEvent>;
