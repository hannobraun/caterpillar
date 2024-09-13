use capi_protocol::command::SerializedCommandToRuntime;
use tokio::sync::mpsc;

pub type CommandsToRuntimeTx =
    mpsc::UnboundedSender<SerializedCommandToRuntime>;
