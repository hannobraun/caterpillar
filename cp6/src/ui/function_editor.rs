use sycamore::{component, reactive::Scope, view, view::View, web::Html, Prop};

use crate::cp;

#[component]
pub fn FunctionEditor<G: Html>(cx: Scope, props: Props) -> View<G> {
    let name = props.function.name;
    let body = format!("{:?}", props.function.body);

    let is_intrinsic =
        matches!(props.function.body, cp::FunctionBody::Intrinsic(_));

    view! { cx,
        div(class="ring-1 rounded mb-4 divide-y") {
            div(class="flex flex-row justify-between") {
                span { (name) }
                (
                    if is_intrinsic {
                        view! { cx, span { "intrinsic" } }
                    }
                    else {
                        view! { cx, }
                    }
                )
            }
            (
                if is_intrinsic {
                    view! { cx, }
                }
                else {
                    view!{ cx, p { (body) } }
                }
            )
        }
    }
}

#[derive(Prop)]
pub struct Props {
    function: cp::Function,
}
