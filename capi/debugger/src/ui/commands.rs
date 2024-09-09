use capi_protocol::command::CommandToRuntime;

use super::{Action, ActionsTx};

pub async fn send_command(command: Action, actions: ActionsTx) {
    let command = match command {
        Action::BreakpointClear { instruction } => {
            CommandToRuntime::BreakpointClear { instruction }
        }
        Action::BreakpointSet { instruction } => {
            CommandToRuntime::BreakpointSet { instruction }
        }
        Action::Continue => CommandToRuntime::Continue,
        Action::Reset => CommandToRuntime::Reset,
        Action::Step => CommandToRuntime::Step,
        Action::Stop => CommandToRuntime::Stop,
    };

    if let Err(err) = actions.send(command) {
        log::error!("Error sending command: {err}");
    }
}
