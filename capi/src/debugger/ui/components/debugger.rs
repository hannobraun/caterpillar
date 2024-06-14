use leptos::{component, view, IntoView, ReadSignal, SignalGet};

use crate::debugger::{
    model::{ActiveFunctions, Debugger},
    ui::{
        components::{
            active_functions::ActiveFunctions, control_panel::ControlPanel,
            memory_explorer::MemoryExplorer, stack_explorer::StackExplorer,
        },
        EventsTx,
    },
};

#[component]
pub fn Debugger(
    debugger: ReadSignal<Debugger>,
    events: EventsTx,
) -> impl IntoView {
    move || {
        let debugger = debugger.get();
        let process = debugger.process;

        let active_functions = ActiveFunctions::new(process.as_ref());

        let stack_explorer = process.as_ref().map(|process| {
            let previous = process.previous_data_stack.clone();
            let current = process.evaluator.data_stack().clone();

            view! {
                <StackExplorer
                    previous=previous
                    current=current />
            }
        });
        let memory_explorer = process.map(|process| {
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
