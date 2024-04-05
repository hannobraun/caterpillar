mod display;
mod runtime;
mod server;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http::trace=info")
        .init();

    let program = runtime::Program::new();

    server::start(program.functions.clone());
    display::run(program)
}
