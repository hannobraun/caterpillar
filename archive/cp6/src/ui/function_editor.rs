use sycamore::{
    component,
    prelude::Indexed,
    reactive::{create_signal, Scope},
    view,
    view::View,
    web::Html,
    Prop,
};

use crate::cp;

#[component]
pub fn FunctionEditor<G: Html>(cx: Scope, props: Props) -> View<G> {
    let name = props.function.name;

    let (body, is_intrinsic) = match props.function.body {
        cp::FunctionBody::Intrinsic(_) => (view! { cx, }, true),
        cp::FunctionBody::UserDefined(body) => {
            let body = format!("{}", cp::Formatter(&body));
            (view! { cx, p { (body) } }, false)
        }
    };

    let mut tags = Vec::new();
    if is_intrinsic {
        tags.push("intrinsic");
    }
    if props.function.is_test {
        tags.push("test");
    }

    let tags = create_signal(cx, tags);

    view! { cx,
        div(class="ring-1 rounded mb-4 divide-y") {
            div(class="flex flex-row justify-between") {
                span(class="font-bold") { (name) }
                div(class="flex flex-row justify-end") {
                    Indexed(
                        iterable=tags,
                        view=|cx, tag| view! { cx,
                            span(
                                class="\
                                    m-1 px-1.5 \
                                    rounded-full bg-yellow-300 \
                                    text-sm"
                            ) {
                                (tag)
                            }
                        }
                    )
                }
            }
            (body)
        }
    }
}

#[derive(Prop)]
pub struct Props {
    function: cp::Function,
}
