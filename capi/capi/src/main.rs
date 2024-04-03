mod capi;
mod display;
mod server;

fn main() -> anyhow::Result<()> {
    let program = capi::Program::new();
    program.functions.print();

    server::start();
    display::run(program)
}
