use capi_runtime::{DataStack, Program};
use leptos::{component, view, IntoView, ReadSignal, SignalGet};

use crate::components::panel::Panel;

#[allow(unused_braces)] // working around a warning from the `view!` macro
#[component]
pub fn StackExplorer(program: ReadSignal<Option<Program>>) -> impl IntoView {
    let data_stack = move || {
        let program = program.get()?;

        let view = view! {
            <Panel class="">
                <div>
                    <p>
                        "Previous data stack: "
                    </p>
                    <DataStack data_stack=program.previous_data_stack />
                </div>
                <div>
                    <p>
                        "Current data stack: "
                    </p>
                    <DataStack data_stack=program.evaluator.data_stack />
                </div>
            </Panel>
        };

        Some(view)
    };

    view! {
        {data_stack}
    }
}

#[component]
pub fn DataStack(data_stack: DataStack) -> impl IntoView {
    view! {
        <p>
            {format!("{:?}", data_stack)}
        </p>
    }
}
