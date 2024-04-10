mod capi;
mod debug;
mod display;
mod server;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http::trace=info")
        .init();

    let (program, functions) = capi::Program::new();

    server::start(debug::DebugState { functions });
    display::run(program)
}
