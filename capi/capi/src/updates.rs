use capi_runtime::Program;
use tokio::sync::watch;

pub type UpdatesRx = watch::Receiver<Program>;

pub struct UpdatesTx {
    inner: UpdatesTxInner,
    pub program_at_client: Option<Program>,
}

impl UpdatesTx {
    pub fn new(inner: UpdatesTxInner) -> Self {
        Self {
            inner,
            program_at_client: None,
        }
    }

    pub fn send(&mut self, program: &Program) {
        self.program_at_client = Some(program.clone());
        self.inner.send(program.clone()).unwrap()
    }
}

pub type UpdatesTxInner = watch::Sender<Program>;
