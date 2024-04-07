use capi_runtime::Function;
use leptos::SignalGet;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging to console");

    let code = leptos::create_local_resource(|| (), fetch_code);
    let code = move || {
        code.get().map(|code| {
            code.into_iter()
                .map(|function| {
                    let mut s = String::new();

                    s.push_str(&function.name);
                    s.push_str(":\n");

                    for line in function.lines {
                        s.push_str("    ");
                        s.push_str(&line);
                        s.push('\n');
                    }

                    s
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
    };

    leptos::mount_to_body(move || {
        leptos::view! {
            <pre>{code}</pre>
        }
    });

    log::info!("Capi Debug initialized.");
}

async fn fetch_code((): ()) -> Vec<FunctionView> {
    let code = reqwest::get("http://127.0.0.1:8080/code")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let code: Vec<Function> = serde_json::from_str(&code).unwrap();

    let mut s = Vec::new();

    for function in code.into_iter() {
        s.push(FunctionView {
            name: function.name,
            lines: function
                .syntax
                .into_iter()
                .map(|syntax| format!("{syntax}"))
                .collect(),
        });
    }

    s
}

#[derive(Clone)]
pub struct FunctionView {
    pub name: String,
    pub lines: Vec<String>,
}
