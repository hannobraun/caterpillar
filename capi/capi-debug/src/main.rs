use leptos::{component, IntoView, SignalGet};

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging to console");

    let code = leptos::create_local_resource(|| (), fetch_code);
    let code = move || {
        code.get().map(|code| {
            code.into_iter()
                .map(|f| leptos::view! { <Function f=f/> })
                .collect::<Vec<_>>()
        })
    };

    leptos::mount_to_body(move || code);

    log::info!("Capi Debug initialized.");
}

#[component]
pub fn function(f: capi_runtime::Function) -> impl IntoView {
    let lines = f
        .syntax
        .into_iter()
        .map(|syntax| format!("    {syntax}\n"))
        .collect::<Vec<_>>()
        .join("");

    leptos::view! {
        <div>
            <div>
                {f.name}:{'\n'}
            </div>
            <pre>
                {lines}
            </pre>
        </div>
    }
}

async fn fetch_code((): ()) -> Vec<capi_runtime::Function> {
    let code = reqwest::get("http://127.0.0.1:8080/code")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str(&code).unwrap()
}
