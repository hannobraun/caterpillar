mod client;
mod execution_context;
mod function;

use capi_runtime::{DebugEvent, Program};
use client::EventsTx;
use futures::channel::mpsc;
use leptos::{
    component, create_signal, view, CollectView, IntoView, ReadSignal,
    SignalGet,
};

use crate::{
    client::{handle_server, send_event},
    execution_context::ExecutionContext,
    function::Function,
};

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
