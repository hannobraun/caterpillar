use leptos::{component, create_memo, view, IntoView, ReadSignal, SignalGet};

use crate::{
    debugger::{
        model::ExecutionContext,
        ui::{
            components::{
                call_stack::CallStack, control_panel::ControlPanel,
                execution_context::ExecutionContext,
                memory_explorer::MemoryExplorer, stack_explorer::StackExplorer,
            },
            EventsTx,
        },
    },
    process::Process,
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
        <div>
            <ControlPanel
                events=events.clone() />
            <CallStack
                process=process
                events=events.clone() />
            <ExecutionContext
                process=process
                execution_context=execution_context
                events=events.clone() />
            <StackExplorer
                process=process />
            <MemoryExplorer
                process=process />
        </div>
    }
}
