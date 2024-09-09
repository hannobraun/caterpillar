use capi_process::InstructionAddress;
use tokio::sync::mpsc;

pub type ActionsTx = mpsc::UnboundedSender<Action>;

#[derive(Clone)]
pub enum Action {
    BreakpointClear { instruction: InstructionAddress },
    BreakpointSet { instruction: InstructionAddress },
    Continue,
    Reset,
    Step,
    Stop,
}

pub async fn send_action(action: Action, actions: ActionsTx) {
    if let Err(err) = actions.send(action) {
        log::error!(
            "Sending a UI action failed, as the receive is no longer \
            available: {err:#?}\n\
            \n\
            This is most likely a bug in the Caterpillar debugger."
        );
    }
}
