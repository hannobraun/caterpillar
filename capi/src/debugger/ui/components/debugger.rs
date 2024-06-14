use leptos::{component, view, IntoView, ReadSignal, SignalGet};

use crate::{
    debugger::{
        model::{ActiveFunctions, ExecutionContext},
        ui::{
            components::{
                active_functions::ActiveFunctions, control_panel::ControlPanel,
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
        let active_functions = ActiveFunctions::new(process.get().as_ref());

        view! {
            <div>
                <ControlPanel
                    events=events.clone() />
                <ActiveFunctions
                    active_functions=active_functions
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
