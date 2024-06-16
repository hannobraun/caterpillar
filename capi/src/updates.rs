use tokio::sync::mpsc;

use crate::{
    process::{Memory, Process},
    syntax,
};

pub fn updates() -> (UpdatesTx, UpdatesRx) {
    let (tx, rx) = mpsc::unbounded_channel();

    let tx = UpdatesTx {
        inner: tx,
        queued_memory: None,
        process_at_client: None,
    };

    (tx, rx)
}

pub type UpdatesRx = mpsc::UnboundedReceiver<Update>;

#[allow(clippy::large_enum_variant)] // haven't optimized this yet
pub enum Update {
    Memory { memory: Memory },
    Process(Process),
    SourceCode { functions: syntax::Functions },
}

#[derive(Clone)]
pub struct UpdatesTx {
    inner: mpsc::UnboundedSender<Update>,
    queued_memory: Option<Memory>,
    process_at_client: Option<Process>,
}

impl UpdatesTx {
    pub fn queue(&mut self, update: Update) {
        match update {
            Update::Memory { memory } => {
                self.queued_memory = Some(memory);
            }
            Update::Process(process) => {
                if self.should_flush(&process) {
                    self.process_at_client = Some(process.clone());
                    self.inner.send(Update::Process(process)).unwrap();

                    self.flush();
                }
            }
            Update::SourceCode { functions } => {
                self.flush();
                self.inner.send(Update::SourceCode { functions }).unwrap();
            }
        }
    }

    fn should_flush(&self, process: &Process) -> bool {
        if let Some(process_at_client) = &self.process_at_client {
            // The client has previously received a program. We don't want to
            // saturate the connection with useless updates, so use that to
            // determine, if we should send an update.

            if process_at_client.can_step() && process.can_step() {
                // While the program is running, sending updates on every change
                // would result in too many updates.
                //
                // Let's check if there's a change that we consider worthy of
                // sending an update for.

                let breakpoints_unchanged =
                    process_at_client.breakpoints == process.breakpoints;

                if breakpoints_unchanged {
                    return false;
                }
            }
        }

        self.process_at_client.as_ref() != Some(process)
    }

    fn flush(&mut self) {
        if let Some(memory) = self.queued_memory.take() {
            self.inner.send(Update::Memory { memory }).unwrap();
        }
    }
}
