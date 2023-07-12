mod cp;
mod repl;
mod test_report;

fn main() -> anyhow::Result<()> {
    let (mut functions, mut tests) = cp::define_code()?;
    repl::run(&mut functions, &mut tests)?;
    Ok(())
}
