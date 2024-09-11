use capi_process::{Effect, InstructionAddress};
use leptos::{
    component, ev::MouseEvent, html::Span, view, wasm_bindgen::JsCast,
    web_sys::HtmlSpanElement, CollectView, HtmlElement, IntoView, View,
};

use crate::{
    model::{Branch, DebugFragment, DebugFragmentData, DebugFragmentKind},
    ui::{Action, ActionsTx},
};

#[component]
pub fn NamedFunction(
    name: String,
    branches: Vec<Branch>,
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
fn Function(branches: Vec<Branch>, actions: ActionsTx) -> impl IntoView {
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
    body: Vec<DebugFragment>,
    actions: ActionsTx,
) -> impl IntoView {
    let parameters = parameters.join(" ");
    let fragments = body
        .into_iter()
        .map(|fragment| {
            view! {
                <li class="ml-8">
                    <Fragment
                        fragment=fragment
                        actions=actions.clone() />
                </li>
            }
        })
        .collect_view();

    view! {
        <div class="pl-8">
            |{parameters}|
            <ol>
                {fragments}
            </ol>
        </div>
    }
}

#[component]
pub fn Fragment(fragment: DebugFragment, actions: ActionsTx) -> impl IntoView {
    let mut class_outer = String::from("py-1");

    let (fragment, error) = match fragment.kind {
        DebugFragmentKind::CallToFunction { name } => make_single_expression(
            name,
            fragment.data,
            &mut class_outer,
            actions,
        ),
        DebugFragmentKind::CallToHostFunction { name } => {
            make_single_expression(
                name,
                fragment.data,
                &mut class_outer,
                actions,
            )
        }
        DebugFragmentKind::CallToIntrinsic { name } => make_single_expression(
            name,
            fragment.data,
            &mut class_outer,
            actions,
        ),
        DebugFragmentKind::Comment { text } => {
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
        DebugFragmentKind::Function { function } => (
            view! {
                <Function
                    branches=function.branches
                    actions=actions />
            }
            .into_view(),
            None,
        ),
        DebugFragmentKind::ResolvedBinding { name } => make_single_expression(
            name,
            fragment.data,
            &mut class_outer,
            actions,
        ),
        DebugFragmentKind::UnresolvedIdentifier { name } => {
            make_single_expression(
                name,
                fragment.data,
                &mut class_outer,
                actions,
            )
        }
        DebugFragmentKind::Value { as_string } => make_single_expression(
            as_string,
            fragment.data,
            &mut class_outer,
            actions,
        ),
    };

    view! {
        <span>
            <span class=class_outer>
                {fragment}
            </span>
            {error}
        </span>
    }
}

fn make_single_expression(
    expression: String,
    data: DebugFragmentData,
    class_outer: &mut String,
    actions: ActionsTx,
) -> (View, Option<HtmlElement<Span>>) {
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
    if data.is_active {
        class_inner.push_str(" font-bold");
    }

    let data_fragment = ron::to_string(&data.id)
        .expect("Expecting serialization of `FragmentId` to always work.");
    let data_instruction =
        data.first_instruction.map(|instruction| instruction.index);
    let data_breakpoint = data.has_durable_breakpoint;

    let error = data.effect.map(|effect| format!("{:?}", effect));

    let toggle_breakpoint = move |event: MouseEvent| {
        let event_target = event.target().unwrap();
        let element = event_target.dyn_ref::<HtmlSpanElement>().unwrap();

        let fragment = {
            let Some(fragment) = element.get_attribute("data-fragment") else {
                // This happens, if the user clicks on a comment.
                return;
            };

            ron::from_str(&fragment).expect(
                "Expecting serialized fragment IDs in DOM to always be valid.",
            )
        };
        let Some(instruction) = element.get_attribute("data-instruction")
        else {
            // This happens, if the user clicks on a comment.
            return;
        };
        let address = InstructionAddress {
            index: instruction
                .parse()
                .expect("Expected `data-instruction` attribute to be a number"),
        };

        let action = if element.has_attribute("data-breakpoint") {
            Action::BreakpointClear { fragment, address }
        } else {
            Action::BreakpointSet { fragment, address }
        };

        leptos::spawn_local(action.send(actions.clone()));
    };

    (
        view! {
            <span
                class=class_inner
                data-fragment=data_fragment
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
