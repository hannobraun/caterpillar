use leptos::{component, view, CollectView, IntoView};

use crate::{
    debugger::{ActiveFunctions, ActiveFunctionsEntry},
    ui::{
        components::{function::NamedFunction, panel::Panel},
        CommandsTx,
    },
};

#[component]
pub fn ActiveFunctions(
    active_functions: ActiveFunctions,
    commands: CommandsTx,
) -> impl IntoView {
    let active_functions = match active_functions {
        ActiveFunctions::Functions { entries: functions } => {
            let functions = functions
                .into_iter()
                .map(|ActiveFunctionsEntry::Function(function)| {
                    let name = function.name.expect(
                        "Only dealing with top-level functions here; should \
                        be named.",
                    );

                    view! {
                        <NamedFunction
                            name=name
                            branches=function.branches
                            commands=commands.clone() />
                    }
                })
                .collect_view();

            view! {
                <ol>
                    {functions}
                </ol>
            }
            .into_view()
        }
        ActiveFunctions::Message { message } => view! {
            <p
                class="w-full h-full absolute inset-y-0 flex \
                    justify-center items-center">
                {message.to_string()}
            </p>
        }
        .into_view(),
    };

    view! {
        <Panel class="h-80">
            {active_functions}
        </Panel>
    }
}
