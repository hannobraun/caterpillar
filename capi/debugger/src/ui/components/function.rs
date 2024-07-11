use capi_process::{BuiltinEffect, EvaluatorEffect};
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
    let fragments = function
        .body
        .into_iter()
        .map(|fragment| {
            view! {
                <li class="ml-8">
                    <Fragment
                        fragment=fragment
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
                {fragments}
            </ol>
        </div>
    }
}

#[component]
pub fn Fragment(fragment: Expression, commands: CommandsTx) -> impl IntoView {
    let mut class_outer = String::from("py-1");
    if fragment.has_durable_breakpoint {
        class_outer.push_str(" bg-blue-300");
    }

    let mut class_inner = String::from("px-0.5");
    if fragment.is_comment {
        class_inner.push_str(" italic text-gray-500");
    }
    if let Some(effect) = &fragment.effect {
        match effect {
            EvaluatorEffect::Builtin(BuiltinEffect::Breakpoint) => {
                class_inner.push_str(" bg-green-300")
            }
            _ => class_inner.push_str(" bg-red-300"),
        }
    }
    if fragment.is_on_call_stack {
        class_inner.push_str(" font-bold");
    }

    let data_location = fragment
        .location
        .map(|location| ron::to_string(&location).unwrap());
    let data_breakpoint = fragment.has_durable_breakpoint;

    let error = fragment.effect.map(|effect| format!("{:?}", effect));

    let toggle_breakpoint = move |event: MouseEvent| {
        let event_target = event.target().unwrap();
        let element = event_target.dyn_ref::<HtmlSpanElement>().unwrap();

        let Some(location) = element.get_attribute("data-location") else {
            // This happens, if the user clicks on a comment.
            return;
        };
        let location = ron::from_str(&location).unwrap();

        let command = if element.has_attribute("data-breakpoint") {
            Command::BreakpointClear { location }
        } else {
            Command::BreakpointSet { location }
        };

        leptos::spawn_local(send_command(command, commands.clone()));
    };

    let expression = format!("{}", fragment.expression);

    view! {
        <span>
            <span class=class_outer>
                <span
                    class=class_inner
                    data-location=data_location
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
