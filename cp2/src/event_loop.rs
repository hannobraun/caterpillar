use crate::terminal;

pub fn run_once(_: terminal::Size) -> anyhow::Result<()> {
    eprintln!("Hello, world!");
    Ok(())
}
