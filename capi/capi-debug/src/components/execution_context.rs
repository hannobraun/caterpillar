use capi_runtime::{Function, Program, ProgramState};
use leptos::{component, view, IntoView, ReadSignal, SignalGet};

use crate::{client::EventsTx, components::function::Function};

#[component]
pub fn ExecutionContext(
    program: ReadSignal<Option<Program>>,
    events: EventsTx,
) -> impl IntoView {
    move || {
        let function = match get_current_function(program) {
            Ok(function) => function,
            Err(error) => {
                return view! {
                    <p>{error}</p>
                }
                .into_view();
            }
        };

        view! {
            <div class="mx-1 my-3 border p-1">
                <Function
                    program=program
                    function=function
                    events=events.clone() />
            </div>
        }
        .into_view()
    }
}

fn get_current_function(
    program: ReadSignal<Option<Program>>,
) -> Result<Function, &'static str> {
    let Some(program) = program.get() else {
        return Err("No program available.");
    };

    let (_effect, address) = match program.state {
        ProgramState::Running => {
            return Err("Program is running.");
        }
        ProgramState::Finished => {
            return Err("Program has finished running.");
        }
        ProgramState::Effect { effect, address } => (effect, address),
    };

    let Some(location) = program.source_map.address_to_location(&address)
    else {
        return Err(
            "Program is stopped at instruction with no associated source \
            location.",
        );
    };

    let function = program
        .functions
        .inner
        .iter()
        .find(|function| function.name == location.function())
        .cloned();
    let Some(function) = function else {
        return Err(
            "Program stopped at unknown function. This is most likely a bug in \
            Caterpillar.",
        );
    };

    Ok(function)
}
