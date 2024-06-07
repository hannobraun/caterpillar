mod call_stack;
mod code_explorer;
mod control_panel;
mod debugger;
mod execution_context;
mod function;
mod memory_explorer;
mod panel;
mod stack_explorer;

pub fn start(
    program: leptos::ReadSignal<Option<capi_runtime::Program>>,
    events_tx: futures::channel::mpsc::UnboundedSender<
        capi_runtime::debugger::DebugEvent,
    >,
) {
    leptos::mount_to_body(move || {
        leptos::view! {
            <debugger::Debugger program=program events=events_tx />
        }
    });
}
