use capi_runtime::Program;
use tokio::sync::watch;

pub type UpdatesRx = watch::Receiver<Program>;

pub struct UpdatesTx {
    inner: UpdatesTxInner,
}

impl UpdatesTx {
    pub fn new(inner: UpdatesTxInner) -> Self {
        Self { inner }
    }

    pub fn send(&self, program: Program) {
        self.inner.send(program).unwrap()
    }
}

pub type UpdatesTxInner = watch::Sender<Program>;
