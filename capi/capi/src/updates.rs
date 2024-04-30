use capi_runtime::Program;
use tokio::sync::watch;

pub type UpdatesRx = watch::Receiver<Program>;
pub type UpdatesTx = watch::Sender<Program>;
