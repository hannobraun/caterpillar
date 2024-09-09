use capi_protocol::command::CommandToRuntime;

use super::ActionsTx;

pub async fn send_command(command: CommandToRuntime, actions: ActionsTx) {
    if let Err(err) = actions.send(command) {
        log::error!("Error sending command: {err}");
    }
}
