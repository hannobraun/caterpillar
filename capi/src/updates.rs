use tokio::sync::mpsc;

use crate::{
    breakpoints, process::Process, source_map::SourceMap, state::Memory, syntax,
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
    Breakpoints {
        event: breakpoints::Event,
    },
    Memory {
        memory: Memory,
    },
    Process(Process),
    SourceCode {
        functions: syntax::Functions,
        source_map: SourceMap,
    },
}

#[derive(Clone)]
pub struct UpdatesTx {
    inner: mpsc::UnboundedSender<Update>,
    queued_memory: Option<Memory>,
    process_at_client: Option<Process>,
}

impl UpdatesTx {
    pub fn send_update_if_necessary(&mut self, process: &mut Process) {
        for event in process.breakpoints.take_events() {
            self.flush();
            self.inner.send(Update::Breakpoints { event }).unwrap();
        }

        if self.should_flush(process) {
            self.process_at_client = Some(process.clone());
            self.inner.send(Update::Process(process.clone())).unwrap();

            self.flush();
        }
    }

    pub fn queue(&mut self, update: Update) {
        match update {
            Update::Breakpoints { .. } => {
                unreachable!();
            }
            Update::Memory { memory } => {
                self.queued_memory = Some(memory);
            }
            Update::Process(_) => {
                unreachable!();
            }
            Update::SourceCode {
                functions,
                source_map,
            } => {
                self.flush();
                self.inner
                    .send(Update::SourceCode {
                        functions,
                        source_map,
                    })
                    .unwrap();
            }
        }
    }

    fn should_flush(&self, process: &Process) -> bool {
        if let Some(process_at_client) = &self.process_at_client {
            // The client has previously received a program. We don't want to
            // saturate the connection with useless updates, so use that to
            // determine, if we should send an update.

            if process_at_client.state().can_step()
                && process.state().can_step()
            {
                // While the program is running, sending updates on every change
                // would result in too many updates.
                return false;
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
