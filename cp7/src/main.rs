mod args;
mod language;
mod loader;

fn main() -> anyhow::Result<()> {
    let example = args::example()?;

    let code = loader::load::load(&example)?;
    let (updates, _watcher) = loader::watch::watch(&example)?;

    language::runtime::start::start(&code, updates)?;
    Ok(())
}
