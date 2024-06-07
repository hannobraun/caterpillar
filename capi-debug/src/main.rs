mod client;
mod ui;

use capi_runtime::{debugger::ExecutionContext, Program};
use futures::channel::mpsc;
use leptos::{
    component, create_memo, create_signal, view, IntoView, ReadSignal,
    SignalGet,
};
use ui::code_explorer::CodeExplorer;

use crate::{
    client::{handle_server, EventsTx},
    ui::{
        call_stack::CallStack, control_panel::ControlPanel,
        execution_context::ExecutionContext, memory_explorer::MemoryExplorer,
        stack_explorer::StackExplorer,
    },
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
    let execution_context = create_memo(move |prev| {
        ExecutionContext::from_program(prev, program.get())
    });

    view! {
        <CallStack
            program=program
            events=events.clone() />
        <ExecutionContext
            program=program
            state=execution_context
            events=events.clone() />
        <ControlPanel
            events=events.clone() />
        <StackExplorer
            program=program />
        <MemoryExplorer
            program=program />
        <CodeExplorer
            program=program
            events=events />
    }
}
