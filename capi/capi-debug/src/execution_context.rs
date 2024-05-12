use capi_runtime::{Program, ProgramState};
use leptos::{component, view, IntoView, ReadSignal, SignalGet};

#[component]
pub fn ExecutionContext(program: ReadSignal<Option<Program>>) -> impl IntoView {
    move || {
        let Some(program) = program.get() else {
            return view! {
                <p>"No program available."</p>
            };
        };

        let (_effect, address) = match program.state {
            ProgramState::Running => {
                return view! {
                    <p>"Program is running."</p>
                }
            }
            ProgramState::Finished => {
                return view! {
                    <p>"Program has finished running."</p>
                }
            }
            ProgramState::Effect { effect, address } => (effect, address),
        };

        let Some(location) = program.source_map.address_to_location(&address)
        else {
            return view! {
                <p>
                    "Program is stopped at instruction with no associated \
                    source location."
                </p>
            };
        };

        let function = program
            .functions
            .inner
            .iter()
            .find(|function| function.name == location.function());
        let Some(_function) = function else {
            return view! {
                <p>
                    "Program stopped at unknown function. This is most likely \
                    a bug in Caterpillar."
                </p>
            };
        };

        view! {
            <p>"Placeholder for execution context"</p>
        }
    }
}
