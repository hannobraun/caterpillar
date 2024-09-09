use capi_protocol::command::CommandToRuntime;

use super::ActionsTx;

pub async fn send_command(command: CommandToRuntime, commands: ActionsTx) {
    if let Err(err) = commands.send(command.serialize()) {
        log::error!("Error sending command: {err}");
    }
}
