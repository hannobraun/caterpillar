mod args;
mod functions;
mod pipeline;
mod runtime;
mod syntax;
mod value;

fn main() -> anyhow::Result<()> {
    let example = args::example()?;
    pipeline::start(example)?;
    Ok(())
}
