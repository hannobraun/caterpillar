mod cp;
mod repl;
mod test_report;

fn main() -> anyhow::Result<()> {
    let mut functions = cp::Functions::new();
    cp::intrinsics::define(&mut functions);
    cp::std::define(&mut functions)?;

    let mut tests = cp::tests::define(&mut functions)?;

    repl::run(&mut functions, &mut tests)?;

    Ok(())
}
