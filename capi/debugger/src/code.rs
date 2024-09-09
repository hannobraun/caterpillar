use capi_process::Instructions;
use tokio::sync::watch;

pub type CodeRx = watch::Receiver<Instructions>;
