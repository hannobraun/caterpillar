mod cp;
mod render;

fn main() -> anyhow::Result<()> {
    console_error_panic_hook::set_once();

    let (mut functions, tests) = cp::define_code()?;
    let test_reports = cp::run_tests(&mut functions, &tests)?;
    render::render(test_reports);

    Ok(())
}
