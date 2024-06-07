mod components;

pub fn start(
    program: leptos::ReadSignal<Option<capi_runtime::Program>>,
    events_tx: futures::channel::mpsc::UnboundedSender<
        capi_runtime::debugger::DebugEvent,
    >,
) {
    leptos::mount_to_body(move || {
        leptos::view! {
            <components::debugger::Debugger program=program events=events_tx />
        }
    });
}
