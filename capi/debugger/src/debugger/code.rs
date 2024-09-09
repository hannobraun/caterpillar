use capi_process::Instructions;
use capi_protocol::updates::Code;
use tokio::sync::watch;

pub type CodeRx = watch::Receiver<Instructions>;
pub type CodeTx = watch::Sender<Instructions>;

pub type DebugCode = Option<Code>;
