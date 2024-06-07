pub mod call_stack;
pub mod code_explorer;
pub mod control_panel;
pub mod debugger;
pub mod execution_context;
pub mod function;
pub mod memory_explorer;
pub mod panel;
pub mod stack_explorer;

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
