use sycamore::{component, reactive::Scope, view, view::View, web::Html};

#[component]
pub fn PassFail<G: Html>(cx: Scope, props: Props) -> View<G> {
    let class = {
        let color = if props.pass {
            "text-green-500"
        } else {
            "text-red-500"
        };

        format!("font-bold mx-2 {color}")
    };
    let text = if props.pass { "PASS" } else { "FAIL" };

    view! { cx,
        span(class=class) { (text) }
    }
}

#[derive(sycamore::Prop)]
pub struct Props {
    pass: bool,
}
