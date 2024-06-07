use capi_runtime::{debugger::ExecutionContext, Program};
use leptos::{component, create_memo, view, IntoView, ReadSignal, SignalGet};

use crate::{
    client::EventsTx,
    ui::components::{
        call_stack::CallStack, code_explorer::CodeExplorer,
        control_panel::ControlPanel, execution_context::ExecutionContext,
        memory_explorer::MemoryExplorer, stack_explorer::StackExplorer,
    },
};

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
