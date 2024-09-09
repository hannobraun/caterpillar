use capi_process::InstructionAddress;
use tokio::sync::mpsc;

pub type ActionsTx = mpsc::UnboundedSender<Action>;

#[derive(Clone)]
pub enum Action {
    BreakpointClear { address: InstructionAddress },
    BreakpointSet { instruction: InstructionAddress },
    Continue,
    Reset,
    Step,
    Stop,
}

impl Action {
    pub async fn send(self, actions: ActionsTx) {
        if let Err(err) = actions.send(self) {
            log::error!(
                "Sending a UI action failed, as the receive is no longer \
                available: {err:#?}\n\
                \n\
                This is most likely a bug in the Caterpillar debugger."
            );
        }
    }
}
