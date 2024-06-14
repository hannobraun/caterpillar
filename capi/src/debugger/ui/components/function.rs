use leptos::{component, view, CollectView, IntoView, ReadSignal, SignalGet};
use web_sys::{wasm_bindgen::JsCast, HtmlSpanElement, MouseEvent};

use crate::{
    debugger::{
        model::{DebugEvent, Expression},
        ui::{send_event, EventsTx},
    },
    process::Process,
    runtime::{BuiltinEffect, EvaluatorEffectKind},
    syntax,
};

#[component]
pub fn Function(
    process: ReadSignal<Option<Process>>,
    function: syntax::Function,
    events: EventsTx,
) -> impl IntoView {
    let expressions = function
        .syntax
        .into_iter()
        .map(|expression| {
            view! {
                <li class="ml-8">
                    <Expression
                        process=process
                        expression=expression
                        events=events.clone() />
                </li>
            }
        })
        .collect_view();

    view! {
        <div class="m-2 mb-4">
            <div class="font-bold">
                {function.name}:{'\n'}
            </div>
            <ol>
                {expressions}
            </ol>
        </div>
    }
}

#[component]
pub fn Expression(
    process: ReadSignal<Option<Process>>,
    expression: syntax::Expression,
    events: EventsTx,
) -> impl IntoView {
    move || {
        // Without this line, this closure turns into an `FnOnce` instead of an
        // `Fn`, and that's no longer an a `leptos::IntoView`.
        let events = events.clone();

        let process = process.get()?;

        let debugger_expression = Expression::new(&expression, &process);

        let mut class_outer = String::from("py-1");
        if debugger_expression.has_durable_breakpoint {
            class_outer.push_str(" bg-blue-300");
        }

        let mut class_inner = String::from("px-0.5");
        if debugger_expression.is_comment {
            class_inner.push_str(" italic text-gray-500");
        }
        if let Some(effect) = &debugger_expression.effect {
            match effect.kind {
                EvaluatorEffectKind::Builtin(BuiltinEffect::Breakpoint) => {
                    class_inner.push_str(" bg-green-300")
                }
                _ => class_inner.push_str(" bg-red-300"),
            }
        }
        if debugger_expression.is_on_call_stack {
            class_inner.push_str(" font-bold");
        }

        let data_location = debugger_expression
            .location
            .map(|location| ron::to_string(&location).unwrap());

        let error = debugger_expression
            .effect
            .map(|effect| format!("{:?}", effect.kind));

        let toggle_breakpoint = move |event: MouseEvent| {
            let event_target = event.target().unwrap();
            let element = event_target.dyn_ref::<HtmlSpanElement>().unwrap();

            let Some(location) = element.get_attribute("data-location") else {
                // This happens, if the user clicks on a comment.
                return;
            };
            let location = ron::from_str(&location).unwrap();

            leptos::spawn_local(send_event(
                DebugEvent::ToggleBreakpoint { location },
                events.clone(),
            ));
        };

        let expression = format!("{}", debugger_expression.kind);

        Some(view! {
            <span>
                <span class=class_outer>
                    <span
                        class=class_inner
                        data-location=data_location
                        on:click=toggle_breakpoint>
                        {expression}
                    </span>
                </span>
                <span class="mx-2 font-bold text-red-800">
                    {error}
                </span>
            </span>
        })
    }
}
