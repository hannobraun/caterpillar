use leptos::{component, view, IntoView, ReadSignal, SignalGet};

use crate::debugger::{
    model::Debugger,
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

        let stack_explorer = debugger.operands.map(|current| {
            view! {
                <StackExplorer
                    current=current />
            }
        });
        let memory_explorer = debugger.memory.map(|memory| {
            view! {
                <MemoryExplorer
                    memory=memory />
            }
        });

        view! {
            <div>
                <ControlPanel
                    events=events.clone() />
                <ActiveFunctions
                    active_functions=debugger.active_functions
                    events=events.clone() />
                {stack_explorer}
                {memory_explorer}
            </div>
        }
    }
}
