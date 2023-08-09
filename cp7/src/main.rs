mod args;
mod language;
mod watcher;

fn main() -> anyhow::Result<()> {
    let example = args::example()?;
    let (updates, _watcher) = watcher::watch(&example)?;
    language::runtime::start::start(example, updates)?;
    Ok(())
}
