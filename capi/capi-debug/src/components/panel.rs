use leptos::{component, view, Children, IntoView};

#[component]
pub fn Panel(children: Children, class: &'static str) -> impl IntoView {
    let class =
        format!("mx-1 my-3 border p-1 relative overflow-y-auto {class}");

    view! {
        <div class=class>
            {children()}
        </div>
    }
}
