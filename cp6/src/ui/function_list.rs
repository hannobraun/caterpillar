use sycamore::{
    component,
    prelude::Indexed,
    reactive::{ReadSignal, Scope},
    view,
    view::View,
    web::Html,
    Prop,
};

use crate::{cp, ui::function_editor::FunctionEditor};

#[component]
pub fn FunctionList<'r, G: Html>(cx: Scope<'r>, props: Props<'r>) -> View<G> {
    let functions = props.functions.map(cx, |functions| {
        functions
            .iter()
            .map(|(_, function)| function)
            .cloned()
            .collect::<Vec<_>>()
    });

    view! { cx,
        ul(class="h-full overflow-auto") {
            Indexed(
                iterable=functions,
                view=|cx, function| view! { cx,
                    li { FunctionEditor(function=function) }
                },
            )
        }
    }
}

#[derive(Prop)]
pub struct Props<'r> {
    functions: &'r ReadSignal<cp::Functions>,
}
