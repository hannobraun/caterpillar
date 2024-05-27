use capi_runtime::Program;
use leptos::{component, view, IntoView, ReadSignal, SignalGet};

#[allow(unused_braces)] // working around a warning from the `view!` macro
#[component]
pub fn MemoryExplorer(program: ReadSignal<Option<Program>>) -> impl IntoView {
    let memory = move || {
        let program = program.get()?;

        let view = view! {
            <div>
                <p>
                    "Current memory: "
                    {format!("{:?}", program.memory)}
                </p>
            </div>
        };

        Some(view)
    };

    view! {
        {memory}
    }
}
