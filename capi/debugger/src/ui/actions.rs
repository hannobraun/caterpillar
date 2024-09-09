use capi_protocol::command::CommandToRuntime;
use tokio::sync::mpsc;

pub type ActionsTx = mpsc::UnboundedSender<CommandToRuntime>;
