use leptos::{component, create_memo, view, IntoView, ReadSignal, SignalGet};

use crate::{
    debugger::ExecutionContext,
    program::Process,
    ui::{
        components::{
            call_stack::CallStack, code_explorer::CodeExplorer,
            control_panel::ControlPanel, execution_context::ExecutionContext,
            memory_explorer::MemoryExplorer, stack_explorer::StackExplorer,
        },
        EventsTx,
    },
};

#[component]
pub fn Debugger(
    process: ReadSignal<Option<Process>>,
    events: EventsTx,
) -> impl IntoView {
    let execution_context = create_memo(move |prev| {
        ExecutionContext::from_process(prev, process.get())
    });

    view! {
        <ControlPanel
            events=events.clone() />
        <CallStack
            process=process
            events=events.clone() />
        <ExecutionContext
            process=process
            state=execution_context
            events=events.clone() />
        <StackExplorer
            process=process />
        <MemoryExplorer
            process=process />
        <CodeExplorer
            process=process
            events=events />
    }
}
