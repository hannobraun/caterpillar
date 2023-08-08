mod args;
mod functions;
mod pipeline;
mod runtime;
mod syntax;
mod value;
mod watcher;

fn main() -> anyhow::Result<()> {
    let example = args::example()?;
    let _watcher = watcher::watch(&example)?;
    runtime::start::start(example)?;
    Ok(())
}
