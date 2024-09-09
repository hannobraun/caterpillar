use leptos::{component, view, IntoView, ReadSignal, SignalGet};

use crate::{
    debugger::Debugger,
    ui::{
        components::{
            active_functions::ActiveFunctions, control_panel::ControlPanel,
            memory_explorer::MemoryExplorer, stack_explorer::StackExplorer,
        },
        ActionsTx,
    },
};

#[component]
pub fn Debugger(
    debugger: ReadSignal<Debugger>,
    commands: ActionsTx,
) -> impl IntoView {
    move || {
        let debugger = debugger.get();

        let stack_explorer = view! {
            <StackExplorer
                current=debugger.operands />
        };
        let memory_explorer = debugger.memory.map(|memory| {
            view! {
                <MemoryExplorer
                    memory=memory />
            }
        });

        view! {
            <div>
                <ControlPanel
                    commands=commands.clone() />
                <ActiveFunctions
                    active_functions=debugger.active_functions
                    commands=commands.clone() />
                {stack_explorer}
                {memory_explorer}
            </div>
        }
    }
}
