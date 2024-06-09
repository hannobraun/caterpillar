use web_sys::js_sys::wasm_bindgen;

mod breakpoints;
mod code;
mod compiler;
mod debugger;
mod display;
mod effects;
mod games;
mod program;
mod runner;
mod runtime;
mod source_map;
mod syntax;
mod ui;
mod updates;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Error)
        .expect("Failed to initialize logging to console");

    leptos::spawn_local(main_async());

    capi_test();
}

async fn main_async() {
    let program = crate::games::build(crate::games::snake::snake);

    let (updates_tx, updates_rx) = crate::updates::updates(program.clone());

    let (events_tx, runner) = {
        let (events_tx, handle, mut runner) =
            crate::runner::runner(program, updates_tx);
        leptos::spawn_local(async move {
            loop {
                runner.step().await;
            }
        });
        (events_tx, handle)
    };

    let set_program = ui::start(events_tx);
    leptos::spawn_local(handle_updates(updates_rx, set_program));

    crate::display::run(runner).await.unwrap();

    log::info!("Caterpillar initialized.");
}

async fn handle_updates(
    mut updates: crate::updates::UpdatesRx,
    set_program: leptos::WriteSignal<Option<crate::program::Program>>,
) {
    use leptos::SignalSet;

    loop {
        let program = match updates.changed().await {
            Ok(()) => updates.borrow_and_update().clone(),
            Err(err) => panic!("{err}"),
        };

        set_program.set(Some(program));
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    fn capi_test();
}
