use sycamore::{component, reactive::Scope, view, view::View, web::Html, Prop};

use crate::cp;

#[component]
pub fn FunctionEditor<G: Html>(cx: Scope, props: Props) -> View<G> {
    let name = props.function.name;
    let body = format!("{:?}", props.function.body);

    view! { cx,
        div {
            p { (name) }
            p { (body) }
        }
    }
}

#[derive(Prop)]
pub struct Props {
    function: cp::Function,
}
