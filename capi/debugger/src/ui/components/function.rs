use capi_runtime::Effect;
use leptos::{
    component, ev::MouseEvent, html::Span, view, wasm_bindgen::JsCast,
    web_sys::HtmlSpanElement, CollectView, HtmlElement, IntoView, View,
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
        DebugExpressionKind::CallToFunction { name } => make_single_expression(
            name,
            expression.data,
            &mut class_outer,
            actions,
        ),
        DebugExpressionKind::CallToFunctionRecursive { name } => {
            make_single_expression(
                name,
                expression.data,
                &mut class_outer,
                actions,
            )
        }
        DebugExpressionKind::CallToHostFunction { name } => {
            make_single_expression(
                name,
                expression.data,
                &mut class_outer,
                actions,
            )
        }
        DebugExpressionKind::CallToIntrinsic { name } => {
            make_single_expression(
                name,
                expression.data,
                &mut class_outer,
                actions,
            )
        }
        DebugExpressionKind::Comment { text } => {
            let class_inner = String::from("italic text-gray-500");
            (
                view! {
                    <span class=class_inner>
                        {text}
                    </span>
                }
                .into_view(),
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
            .into_view(),
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
        DebugExpressionKind::UnresolvedLocalFunction => make_single_expression(
            "unresolved local function".into(),
            expression.data,
            &mut class_outer,
            actions,
        ),
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
) -> (View, Option<HtmlElement<Span>>, Option<HtmlElement<Span>>) {
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
        Some(view! {
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
        })
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

        leptos::spawn_local(send_action(action, actions_tx.clone()));
    };

    (
        view! {
            <span
                class=class_inner
                data-expression=data_expression
                data-breakpoint=data_breakpoint
                on:click=toggle_breakpoint>
                {expression}
            </span>
        }
        .into_view(),
        actions,
        Some(view! {
            <span class="mx-2 font-bold text-red-800">
                {error}
            </span>
        }),
    )
}
