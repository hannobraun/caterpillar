extern crate capi_runtime as runtime;

mod capi;
mod display;
mod server;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tower_http::trace=info")
        .init();

    let program = capi::Program::new();

    server::start(program.functions.clone());
    display::run(program)
}
