mod args;
mod language;
mod loader;

fn main() -> anyhow::Result<()> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .finish(),
    )?;

    let example = args::example()?;

    let code = loader::load::load(&example)?;
    let (updates, _watcher) = loader::watch::watch(&example)?;

    language::runtime::start::start(&code, updates)?;
    Ok(())
}
