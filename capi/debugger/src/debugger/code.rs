use capi_process::Instructions;
use capi_protocol::updates::Code;
use tokio::sync::watch;

pub type CodeRx = watch::Receiver<Instructions>;
pub type CodeTx = watch::Sender<Instructions>;

#[derive(Clone, Debug, Default)]
pub struct DebugCode {
    pub inner: Option<Code>,
}

impl DebugCode {
    pub fn on_new_code(&mut self, code: Code) {
        self.inner = Some(code);
    }
}
