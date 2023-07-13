mod cp;
mod ui;

fn main() -> anyhow::Result<()> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Error initializing logging");

    let mut test_runner = cp::TestRunner::new()?;
    let test_reports = test_runner.run_tests();
    ui::render(test_reports);

    Ok(())
}
