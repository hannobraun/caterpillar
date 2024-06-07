use leptos::SignalSet;

mod ui;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info)
        .expect("Failed to initialize logging to console");

    leptos::spawn_local(main_async());
}

async fn main_async() {
    let program = capi_runtime::games::build(capi_runtime::games::snake::snake);

    let (updates_tx, updates_rx) =
        capi_runtime::updates::updates(program.clone());

    let (events_tx, runner) = {
        let (events_tx, handle, mut runner) =
            capi_runtime::runner::runner(program, updates_tx);
        leptos::spawn_local(async move {
            loop {
                runner.step().await;
            }
        });
        (events_tx, handle)
    };

    let set_program = ui::start(events_tx);
    leptos::spawn_local(handle_updates(updates_rx, set_program));

    capi_runtime::display::run(runner).await.unwrap();

    log::info!("Caterpillar initialized.");
}

async fn handle_updates(
    mut updates: capi_runtime::updates::UpdatesRx,
    set_program: leptos::WriteSignal<Option<capi_runtime::Program>>,
) {
    loop {
        let program = match updates.changed().await {
            Ok(()) => updates.borrow_and_update().clone(),
            Err(err) => panic!("{err}"),
        };

        set_program.set(Some(program));
    }
}
