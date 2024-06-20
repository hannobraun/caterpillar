use tokio::sync::mpsc;

use crate::{
    breakpoints::{self, Breakpoints},
    process::{self, Process},
    runtime,
    source_map::SourceMap,
    state::Memory,
    syntax,
};

pub fn updates() -> (UpdatesTx, UpdatesRx) {
    let (tx, rx) = mpsc::unbounded_channel();

    let tx = UpdatesTx {
        inner: tx,
        queued_memory: None,
        queued_step: None,
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
    Process2 {
        event: process::Event,
    },
    SourceCode {
        functions: syntax::Functions,
        source_map: SourceMap,
    },
}

#[derive(Clone)]
pub struct UpdatesTx {
    inner: mpsc::UnboundedSender<Update>,
    queued_memory: Option<Memory>,
    queued_step: Option<runtime::Location>,
    process_at_client: Option<Process>,
}

impl UpdatesTx {
    pub fn send_update_if_necessary(
        &mut self,
        breakpoints: &mut Breakpoints,
        process: &mut Process,
    ) {
        for event in breakpoints.take_events() {
            self.flush();
            self.inner.send(Update::Breakpoints { event }).unwrap();
        }

        let process_can_step = process.state().can_step();
        for event in process.take_events() {
            if let process::Event::HasStepped { location } = &event {
                // We can't send every step, or we would overwhelm the
                // connection. Let's queue this latest step in any case.
                self.queued_step = Some(location.clone());

                if process_can_step {
                    // Looks like the process is still at it. Don't do
                    // anything further, until something else happens.
                    //
                    // By `break`ing here, we drop any events that come after
                    // this step, which _must_ be a bug. It seems like this
                    // really think this should be a `continue` instead, but
                    // that causes the whole page to freeze, for reasons that
                    // are unclear.
                    //
                    // Maybe this problem will resolve itself as part of the
                    // ongoing (at the time of writing) cleanup. If not, this
                    // requires deeper inspection.
                    break;
                }
            }

            // If we make it there, then either this is a step that we don't
            // want to leave queued, or it's another process event, all of
            // which we want to send immediately.

            self.flush();
            self.inner.send(Update::Process2 { event }).unwrap();
        }

        for _ in process.take_stack_events() {
            // not handled yet
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
            Update::Process(process) => {
                if self.should_flush(&process) {
                    self.process_at_client = Some(process.clone());
                    self.inner.send(Update::Process(process)).unwrap();

                    self.flush();
                }
            }
            Update::Process2 { .. } => {
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
        if let Some(location) = self.queued_step.take() {
            self.inner
                .send(Update::Process2 {
                    event: process::Event::HasStepped { location },
                })
                .unwrap();
        }
    }
}
