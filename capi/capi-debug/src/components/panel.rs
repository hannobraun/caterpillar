use leptos::{component, view, Children, IntoView};

#[component]
pub fn Panel(children: Children) -> impl IntoView {
    view! {
        <div class="mx-1 my-3 border p-1 relative">
            {children()}
        </div>
    }
}
