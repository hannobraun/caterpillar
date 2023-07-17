use sycamore::{component, reactive::Scope, view, view::View, web::Html, Prop};

use crate::cp;

#[component]
pub fn FunctionEditor<G: Html>(cx: Scope, props: Props) -> View<G> {
    let body = format!("{:?}", props.function.body);

    view! { cx,
        div {
            p { (props.function.name) }
            p { (body) }
        }
    }
}

#[derive(Prop)]
pub struct Props {
    function: cp::Function,
}
