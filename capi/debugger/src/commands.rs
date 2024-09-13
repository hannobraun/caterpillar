use capi_protocol::command::SerializedCommandToRuntime;
use tokio::sync::mpsc::UnboundedSender;

pub type CommandsToRuntimeTx = UnboundedSender<SerializedCommandToRuntime>;
