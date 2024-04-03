mod capi;
mod display;

fn main() -> anyhow::Result<()> {
    display::run()
}
