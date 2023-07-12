mod cp;
mod ui;

fn main() -> anyhow::Result<()> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Error initializing logging");

    let (mut functions, tests) = cp::define_code()?;
    let test_reports = cp::run_tests(&mut functions, &tests)?;
    ui::render(test_reports);

    Ok(())
}
