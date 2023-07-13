mod cp;
mod ui;

fn main() -> anyhow::Result<()> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Error initializing logging");

    let test_runner = cp::TestRunner::new()?;
    ui::render(test_runner);

    Ok(())
}
