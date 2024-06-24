use capi_protocol::command::SerializedCommand;
use tokio::sync::mpsc;

use crate::debugger::model::Command;

pub type CommandsRx = mpsc::UnboundedReceiver<SerializedCommand>;
pub type CommandsTx = mpsc::UnboundedSender<SerializedCommand>;

pub async fn send_command(command: Command, commands: CommandsTx) {
    if let Err(err) = commands.send(command.serialize()) {
        log::error!("Error sending command: {err}");
    }
}
