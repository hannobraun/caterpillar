use leptos::{component, view, IntoView, ReadSignal, SignalGet};

use crate::{
    debugger::{
        model::ActiveFunctions,
        ui::{
            components::{
                active_functions::ActiveFunctions, control_panel::ControlPanel,
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
        let active_functions = ActiveFunctions::new(process.get().as_ref());

        let stack_explorer = process.get().map(|process| {
            view! {
                <StackExplorer
                    process=process />
            }
        });
        let memory_explorer = process.get().map(|process| {
            view! {
                <MemoryExplorer
                    memory=process.memory />
            }
        });

        view! {
            <div>
                <ControlPanel
                    events=events.clone() />
                <ActiveFunctions
                    active_functions=active_functions
                    events=events.clone() />
                {stack_explorer}
                {memory_explorer}
            </div>
        }
    }
}
