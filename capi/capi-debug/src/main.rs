fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging to console");

    leptos::mount_to_body(|| {
        leptos::view! {
            <p>Hello, world!</p>
        }
    });

    log::info!("Capi Debug initialized.");
}
