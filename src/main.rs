mod args;
mod loader;

pub use capi_core as language;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = args::Args::parse();
    let code = loader::load::load(&args.script)?;
    let (updates, _watcher) = loader::watch::watch(&args.script)?;
    language::runtime::start::start(&code, updates)?;

    Ok(())
}
