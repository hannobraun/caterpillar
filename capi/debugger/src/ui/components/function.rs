use capi_process::{Effect, InstructionAddress};
use capi_protocol::command::Command;
use leptos::{
    component, ev::MouseEvent, view, wasm_bindgen::JsCast,
    web_sys::HtmlSpanElement, CollectView, IntoView,
};

use crate::{
    debugger::{Branch, Expression, OtherExpression},
    ui::{send_command, CommandsTx},
};

#[component]
pub fn NamedFunction(
    name: String,
    branches: Vec<Branch>,
    commands: CommandsTx,
) -> impl IntoView {
    view! {
        <div class="m-2 mb-4">
            <div class="font-bold">
                {name}:
            </div>
            <Function
                branches=branches
                commands=commands />
        </div>
    }
}

#[component]
fn Function(branches: Vec<Branch>, commands: CommandsTx) -> impl IntoView {
    let branches = branches
        .into_iter()
        .map(|branch| {
            view! {
                <Branch
                    parameters=branch.parameters
                    expressions=branch.body
                    commands=commands.clone() />
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div>
            "{"
            <ol>
                {branches}
            </ol>
            "}"
        </div>
    }
}

#[component]
fn Branch(
    parameters: Vec<String>,
    expressions: Vec<Expression>,
    commands: CommandsTx,
) -> impl IntoView {
    let parameters = parameters.join(" ");
    let expressions = expressions
        .into_iter()
        .map(|fragment| {
            view! {
                <li class="ml-8">
                    <Expression
                        expression=fragment
                        commands=commands.clone() />
                </li>
            }
        })
        .collect_view();

    view! {
        <div>
            |{parameters}|
            <ol>
                {expressions}
            </ol>
        </div>
    }
}

#[component]
pub fn Expression(
    expression: Expression,
    commands: CommandsTx,
) -> impl IntoView {
    let mut class_outer = String::from("py-1");

    let (expression, error) = match expression {
        Expression::Comment { text } => {
            let class_inner = String::from("italic text-gray-500");

            (
                view! {
                    <span class=class_inner>
                        {text}
                    </span>
                }
                .into_view(),
                None,
            )
        }
        Expression::Function { function } => (
            view! {
                <Function
                    branches=function.branches
                    commands=commands />
            }
            .into_view(),
            None,
        ),
        Expression::Other(OtherExpression {
            expression,
            first_instruction,
            has_durable_breakpoint,
            is_on_call_stack,
            effect,
        }) => {
            if has_durable_breakpoint {
                class_outer.push_str(" bg-blue-300");
            }

            let mut class_inner = String::from("px-0.5");
            if let Some(effect) = &effect {
                match effect {
                    Effect::Breakpoint => class_inner.push_str(" bg-green-300"),
                    _ => class_inner.push_str(" bg-red-300"),
                }
            }
            if is_on_call_stack {
                class_inner.push_str(" font-bold");
            }

            let data_instruction =
                first_instruction.map(|instruction| instruction.index);
            let data_breakpoint = has_durable_breakpoint;

            let error = effect.map(|effect| format!("{:?}", effect));

            let toggle_breakpoint = move |event: MouseEvent| {
                let event_target = event.target().unwrap();
                let element =
                    event_target.dyn_ref::<HtmlSpanElement>().unwrap();

                let Some(instruction) =
                    element.get_attribute("data-instruction")
                else {
                    // This happens, if the user clicks on a comment.
                    return;
                };
                let instruction = InstructionAddress {
                    index: instruction.parse().expect(
                        "Expected `data-instruction` attribute to be a number",
                    ),
                };

                let command = if element.has_attribute("data-breakpoint") {
                    Command::BreakpointClear { instruction }
                } else {
                    Command::BreakpointSet { instruction }
                };

                leptos::spawn_local(send_command(command, commands.clone()));
            };

            let expression = format!("{expression}");

            (
                view! {
                    <span
                        class=class_inner
                        data-instruction=data_instruction
                        data-breakpoint=data_breakpoint
                        on:click=toggle_breakpoint>
                        {expression}
                    </span>
                }
                .into_view(),
                Some(view! {
                    <span class="mx-2 font-bold text-red-800">
                        {error}
                    </span>
                }),
            )
        }
    };

    view! {
        <span>
            <span class=class_outer>
                {expression}
            </span>
            {error}
        </span>
    }
}
