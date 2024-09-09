use capi_protocol::command::SerializedCommandToRuntime;
use tokio::sync::mpsc;

pub type ActionsTx = mpsc::UnboundedSender<SerializedCommandToRuntime>;
