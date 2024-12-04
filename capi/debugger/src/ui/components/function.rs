use std::fmt::{self, Write};

use capi_compiler::code::syntax::{Signature, Type};
use capi_runtime::Effect;
use leptos::{
    component,
    ev::MouseEvent,
    prelude::{
        AnyView, ClassAttribute, CollectView, CustomAttribute, ElementChild,
        IntoAny, OnAttribute,
    },
    view,
    wasm_bindgen::JsCast,
    web_sys::HtmlSpanElement,
    IntoView,
};

use crate::{
    model::{
        DebugBranch, DebugExpression, DebugExpressionData, DebugExpressionKind,
        UserAction,
    },
    ui::{actions::send_action, ActionsTx},
};

use super::button::Button;

#[component]
pub fn NamedFunction(
    name: String,
    branches: Vec<DebugBranch>,
    actions: ActionsTx,
) -> impl IntoView {
    view! {
        <div class="m-2 mb-4">
            <div class="font-bold">
                {name}:
            </div>
            <Function
                branches=branches
                actions=actions />
        </div>
    }
}

#[component]
fn Function(branches: Vec<DebugBranch>, actions: ActionsTx) -> impl IntoView {
    let branches = branches
        .into_iter()
        .map(|branch| {
            view! {
                <Branch
                    parameters=branch.parameters
                    body=branch.body
                    actions=actions.clone() />
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
    body: Vec<DebugExpression>,
    actions: ActionsTx,
) -> impl IntoView {
    let parameters = parameters.join(" ");
    let expressions = body
        .into_iter()
        .map(|expression| {
            view! {
                <li class="ml-8">
                    <Expression
                        expression=expression
                        actions=actions.clone() />
                </li>
            }
        })
        .collect_view();

    view! {
        <div class="pl-8">
            |{parameters}|
            <ol>
                {expressions}
            </ol>
        </div>
    }
}

#[component]
pub fn Expression(
    expression: DebugExpression,
    actions: ActionsTx,
) -> impl IntoView {
    let mut class_outer = String::from("py-1");

    let (expression, actions, error) = match expression.kind {
        DebugExpressionKind::Comment { text } => {
            let class_inner = String::from("italic text-gray-500");
            (
                view! {
                    <span class=class_inner>
                        {text}
                    </span>
                }
                .into_any(),
                None,
                None,
            )
        }
        DebugExpressionKind::Function { function } => (
            view! {
                <Function
                    branches=function.branches
                    actions=actions />
            }
            .into_any(),
            None,
            None,
        ),
        DebugExpressionKind::UnresolvedIdentifier { name } => {
            make_single_expression(
                name,
                expression.data,
                &mut class_outer,
                actions,
            )
        }
        DebugExpressionKind::Value { as_string } => make_single_expression(
            as_string,
            expression.data,
            &mut class_outer,
            actions,
        ),
    };

    view! {
        <span>
            <span class=class_outer>
                {expression}
            </span>
            {actions}
            {error}
        </span>
    }
}

fn make_single_expression(
    expression: String,
    data: DebugExpressionData,
    class_outer: &mut String,
    actions_tx: ActionsTx,
) -> (AnyView, Option<AnyView>, Option<AnyView>) {
    if data.has_durable_breakpoint {
        class_outer.push_str(" bg-blue-300");
    }

    let mut class_inner = String::from("px-0.5");
    if let Some(effect) = &data.effect {
        match effect {
            Effect::Breakpoint => class_inner.push_str(" bg-green-300"),
            _ => class_inner.push_str(" bg-red-300"),
        }
    }
    if data.state.is_active() {
        class_inner.push_str(" font-bold");
    }

    let data_expression = ron::to_string(&data.location).expect(
        "Expecting serialization of `ExpressionLocation` to always work.",
    );
    let data_breakpoint = data.has_durable_breakpoint;

    let actions = if data.state.is_innermost_active_expression() {
        Some(
            view! {
                <span>
                    <Button
                        label="Step In"
                        action=UserAction::StepIn
                        actions=actions_tx.clone() />
                    <Button
                        label="Step Out"
                        action=UserAction::StepOut
                        actions=actions_tx.clone() />
                    <Button
                        label="Step Over"
                        action=UserAction::StepOver
                        actions=actions_tx.clone() />
                </span>
            }
            .into_any(),
        )
    } else {
        None
    };

    let error = data.effect.map(|effect| format!("{:?}", effect));

    let toggle_breakpoint = move |event: MouseEvent| {
        let event_target = event.target().unwrap();
        let element = event_target.dyn_ref::<HtmlSpanElement>().unwrap();

        let expression = {
            let Some(expression) = element.get_attribute("data-expression")
            else {
                // This happens, if the user clicks on a comment.
                return;
            };

            ron::from_str(&expression).expect(
                "Expecting serialized expression locations in DOM to always be \
                valid.",
            )
        };

        let action = if element.has_attribute("data-breakpoint") {
            UserAction::BreakpointClear { expression }
        } else {
            UserAction::BreakpointSet { expression }
        };

        leptos::task::spawn_local(send_action(action, actions_tx.clone()));
    };

    let mut typed_expression = expression;
    if let Some(signature) = data.signature {
        render_signature(&mut typed_expression, signature)
            .expect("Writing to `String` can't fail.");
    }

    (
        view! {
            <span
                class=class_inner
                data-expression=data_expression
                data-breakpoint=data_breakpoint
                on:click=toggle_breakpoint>
                {typed_expression}
            </span>
        }
        .into_any(),
        actions,
        Some(
            view! {
                <span class="mx-2 font-bold text-red-800">
                    {error}
                </span>
            }
            .into_any(),
        ),
    )
}

fn render_signature(s: &mut String, signature: Signature) -> fmt::Result {
    write!(s, ": ")?;

    let mut inputs = signature.inputs.into_iter().peekable();
    while let Some(input) = inputs.next() {
        render_type(s, input)?;

        if inputs.peek().is_some() {
            write!(s, ", ")?;
        }
    }

    write!(s, "->")?;

    for output in signature.outputs {
        render_type(s, output)?;
    }

    write!(s, " .")?;

    Ok(())
}

fn render_type(s: &mut String, type_: Type) -> fmt::Result {
    match type_ {
        Type::Function { .. } => {
            write!(s, "{type_:?} ")?;
        }
        Type::Identifier { name } => {
            write!(s, "{name}")?;
        }
    }

    Ok(())
}
