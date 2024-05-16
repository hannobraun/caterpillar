use capi_runtime::Program;
use tokio::sync::watch;

pub type UpdatesRx = watch::Receiver<Program>;

pub struct UpdatesTx {
    inner: UpdatesTxInner,
    program_at_client: Option<Program>,
}

impl UpdatesTx {
    pub fn new(inner: UpdatesTxInner) -> Self {
        Self {
            inner,
            program_at_client: None,
        }
    }

    pub fn send_if_relevant_change(&mut self, program: &Program) {
        if let Some(program_at_client) = &self.program_at_client {
            // The client has previously received a program. We don't want to
            // saturate the connection with useless updates, so use that to
            // determine, if we should send an update.

            if program_at_client.state.is_running()
                && program.state.is_running()
            {
                // Don't send updates when the program is running. This would
                // result in too many updates, at a too rapid rate to be useful.
                return;
            }
        }
        if self.program_at_client.as_ref() == Some(program) {
            // Client already has this program. Don't need to send it again.
            return;
        }

        self.program_at_client = Some(program.clone());
        self.inner.send(program.clone()).unwrap();
    }
}

pub type UpdatesTxInner = watch::Sender<Program>;
