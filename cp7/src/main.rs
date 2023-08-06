mod args;
mod data_stack;
mod functions;
mod pipeline;
mod value;

fn main() -> anyhow::Result<()> {
    let example = args::example()?;
    pipeline::run(example)?;
    Ok(())
}
