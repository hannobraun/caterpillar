use tokio::sync::mpsc;

use crate::{process::Process, source_map::SourceMap, state::Memory, syntax};

pub fn updates() -> (UpdatesTx, UpdatesRx) {
    let (tx, rx) = mpsc::unbounded_channel();

    let tx = UpdatesTx {
        latest_memory: None,
        process_at_client: None,
        transport: Transport { channel: tx },
    };

    (tx, rx)
}

pub type UpdatesRx = mpsc::UnboundedReceiver<Update>;

#[allow(clippy::large_enum_variant)] // haven't optimized this yet
pub enum Update {
    Memory {
        memory: Memory,
    },
    Process(Process),
    SourceCode {
        functions: syntax::Functions,
        source_map: SourceMap,
    },
}

pub struct UpdatesTx {
    latest_memory: Option<Memory>,
    process_at_client: Option<Process>,
    transport: Transport,
}

impl UpdatesTx {
    pub fn send_source_code(
        &mut self,
        functions: syntax::Functions,
        source_map: SourceMap,
    ) {
        self.transport.send(Update::SourceCode {
            functions,
            source_map,
        });
    }

    pub fn send_update_if_necessary(
        &mut self,
        process: &Process,
        memory: &Memory,
    ) {
        self.latest_memory = Some(memory.clone());

        if self.update_is_necessary(process) {
            self.process_at_client = Some(process.clone());
            self.transport.send(Update::Process(process.clone()));

            if let Some(memory) = self.latest_memory.take() {
                self.transport.send(Update::Memory { memory });
            }
        }
    }

    fn update_is_necessary(&self, process: &Process) -> bool {
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
}

struct Transport {
    channel: mpsc::UnboundedSender<Update>,
}

impl Transport {
    pub fn send(&mut self, update: Update) {
        self.channel.send(update).unwrap();
    }
}
