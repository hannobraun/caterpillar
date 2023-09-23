mod args;
mod language;
mod loader;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let example = args::Args::parse().script;

    let code = loader::load::load(&example)?;
    let (updates, _watcher) = loader::watch::watch(&example)?;

    language::runtime::start::start(&code, updates)?;
    Ok(())
}
