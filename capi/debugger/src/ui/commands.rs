use capi_protocol::command::{CommandToRuntime, SerializedCommandToRuntime};
use tokio::sync::mpsc;

pub type CommandsTx = mpsc::UnboundedSender<SerializedCommandToRuntime>;

pub async fn send_command(command: CommandToRuntime, commands: CommandsTx) {
    if let Err(err) = commands.send(command.serialize()) {
        log::error!("Error sending command: {err}");
    }
}
