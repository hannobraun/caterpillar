use leptos::{component, view, IntoView, ReadSignal, SignalGet};

use crate::{
    debugger::{
        model::ExecutionContext,
        ui::{
            components::{
                call_stack::ActiveFunctions, control_panel::ControlPanel,
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
    move || {
        let execution_context = ExecutionContext::from_process(process.get());

        view! {
            <div>
                <ControlPanel
                    events=events.clone() />
                <ActiveFunctions
                    process=process
                    events=events.clone() />
                <ExecutionContext
                    execution_context=execution_context
                    events=events.clone() />
                <StackExplorer
                    process=process />
                <MemoryExplorer
                    process=process />
            </div>
        }
    }
}
