use tokio::sync::mpsc;

use crate::debugger::model::{DebugCommand, SerializedCommand};

pub type CommandsRx = mpsc::UnboundedReceiver<SerializedCommand>;
pub type CommandsTx = mpsc::UnboundedSender<SerializedCommand>;

pub async fn send_command(command: DebugCommand, commands: CommandsTx) {
    if let Err(err) = commands.send(command.serialize()) {
        log::error!("Error sending command: {err}");
    }
}
