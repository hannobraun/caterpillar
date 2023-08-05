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
    let function_list = props.functions.map(cx, |(functions, tests)| {
        let mut function_list = Vec::new();

        function_list
            .extend(functions.iter().map(|(_, function)| function).cloned());
        function_list
            .extend(tests.iter().map(|(_, function)| function).cloned());

        function_list
    });

    view! { cx,
        ul {
            Indexed(
                iterable=function_list,
                view=|cx, function| view! { cx,
                    li { FunctionEditor(function=function) }
                },
            )
        }
    }
}

#[derive(Prop)]
pub struct Props<'r> {
    functions: &'r ReadSignal<(cp::Functions, cp::Functions)>,
}
