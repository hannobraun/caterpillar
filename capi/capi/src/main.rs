mod capi;
mod display;
mod server;

fn main() -> anyhow::Result<()> {
    server::start();
    display::run()
}
