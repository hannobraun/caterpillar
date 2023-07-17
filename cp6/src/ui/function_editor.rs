use sycamore::{component, reactive::Scope, view, view::View, web::Html, Prop};

use crate::cp;

#[component]
pub fn FunctionEditor<G: Html>(cx: Scope, props: Props) -> View<G> {
    view! { cx,
        (props.function.name)
    }
}

#[derive(Prop)]
pub struct Props {
    function: cp::Function,
}
