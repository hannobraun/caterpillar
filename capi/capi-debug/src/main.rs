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
                    let lines = function
                        .lines
                        .into_iter()
                        .map(|line| format!("    {line}\n"))
                        .collect::<Vec<_>>()
                        .join("");

                    leptos::view! {
                        <div>
                            <pre>
                                {function.name}:{'\n'}
                                {lines}
                            </pre>
                        </div>
                    }
                })
                .collect::<Vec<_>>()
        })
    };

    leptos::mount_to_body(move || code);

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
    code.into_iter().map(Into::into).collect()
}

#[derive(Clone)]
pub struct FunctionView {
    pub name: String,
    pub lines: Vec<String>,
}

impl From<Function> for FunctionView {
    fn from(Function { name, syntax }: Function) -> Self {
        Self {
            name,
            lines: syntax
                .into_iter()
                .map(|syntax| format!("{syntax}"))
                .collect(),
        }
    }
}
