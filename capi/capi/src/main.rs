mod capi;
mod display;
mod server;

fn main() -> anyhow::Result<()> {
    let program = capi::Program::new();

    server::start(program.functions.clone());
    display::run(program)
}
