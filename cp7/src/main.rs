mod args;
mod language;
mod loader;

fn main() -> anyhow::Result<()> {
    let example = args::example()?;
    let (updates, _watcher) = loader::watch(&example)?;
    language::runtime::start::start(example, updates)?;
    Ok(())
}
