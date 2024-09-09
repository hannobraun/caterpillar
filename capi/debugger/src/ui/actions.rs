use capi_protocol::command::CommandToRuntime;
use tokio::sync::mpsc;

pub type ActionsTx = mpsc::UnboundedSender<CommandToRuntime>;

pub enum Action {
    Continue,
    Reset,
    Step,
    Stop,
}
