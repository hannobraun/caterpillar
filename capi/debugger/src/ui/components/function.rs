use capi_process::{EvaluatorEffect, InstructionAddr};
use capi_protocol::command::Command;
use leptos::{
    component, ev::MouseEvent, view, wasm_bindgen::JsCast,
    web_sys::HtmlSpanElement, CollectView, IntoView,
};

use crate::{
    model::{Expression, Function},
    ui::{send_command, CommandsTx},
};

#[component]
pub fn Function(function: Function, commands: CommandsTx) -> impl IntoView {
    let expressions = function
        .body
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
        <div class="m-2 mb-4">
            <div class="font-bold">
                {function.name}:
            </div>
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
    if expression.has_durable_breakpoint {
        class_outer.push_str(" bg-blue-300");
    }

    let mut class_inner = String::from("px-0.5");
    if expression.is_comment {
        class_inner.push_str(" italic text-gray-500");
    }
    if let Some(effect) = &expression.effect {
        match effect {
            EvaluatorEffect::Breakpoint => {
                class_inner.push_str(" bg-green-300")
            }
            _ => class_inner.push_str(" bg-red-300"),
        }
    }
    if expression.is_on_call_stack {
        class_inner.push_str(" font-bold");
    }

    let data_instruction =
        expression.instruction.map(|instruction| instruction.index);
    let data_breakpoint = expression.has_durable_breakpoint;

    let error = expression.effect.map(|effect| format!("{:?}", effect));

    let toggle_breakpoint = move |event: MouseEvent| {
        let event_target = event.target().unwrap();
        let element = event_target.dyn_ref::<HtmlSpanElement>().unwrap();

        let Some(instruction) = element.get_attribute("data-instruction")
        else {
            // This happens, if the user clicks on a comment.
            return;
        };
        let instruction = InstructionAddr {
            index: instruction
                .parse()
                .expect("Expected `data-instruction` attribute to be a number"),
        };

        let command = if element.has_attribute("data-breakpoint") {
            Command::BreakpointClear { instruction }
        } else {
            Command::BreakpointSet { instruction }
        };

        leptos::spawn_local(send_command(command, commands.clone()));
    };

    let expression = format!("{}", expression.expression);

    view! {
        <span>
            <span class=class_outer>
                <span
                    class=class_inner
                    data-instruction=data_instruction
                    data-breakpoint=data_breakpoint
                    on:click=toggle_breakpoint>
                    {expression}
                </span>
            </span>
            <span class="mx-2 font-bold text-red-800">
                {error}
            </span>
        </span>
    }
}
