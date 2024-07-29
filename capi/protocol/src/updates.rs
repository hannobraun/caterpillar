use capi_compiler::{repr::fragments::Fragments, source_map::SourceMap};
use capi_process::Process;

use crate::{host::GameEngineHost, memory::Memory};

#[derive(Default)]
pub struct Updates {
    latest_memory: Option<Memory>,
    process_at_client: Option<Process<GameEngineHost>>,
    queue: Vec<Update>,
}

impl Updates {
    pub fn queue_updates(
        &mut self,
        process: &Process<GameEngineHost>,
        memory: &Memory,
    ) {
        self.latest_memory = Some(memory.clone());

        if self.update_is_necessary(process) {
            self.process_at_client = Some(process.clone());
            self.queue.push(Update::Process(process.clone()));

            if let Some(memory) = self.latest_memory.take() {
                self.queue.push(Update::Memory { memory });
            }
        }
    }

    pub fn take_queued_updates(&mut self) -> impl Iterator<Item = Update> + '_ {
        self.queue.drain(..)
    }

    fn update_is_necessary(&self, process: &Process<GameEngineHost>) -> bool {
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

#[allow(clippy::large_enum_variant)] // haven't optimized this yet
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Update {
    Process(Process<GameEngineHost>),
    Memory { memory: Memory },
}

impl Update {
    pub fn deserialize(bytes: SerializedUpdate) -> Self {
        let string = std::str::from_utf8(&bytes).unwrap();
        ron::from_str(string).unwrap()
    }

    pub fn serialize(&self) -> SerializedUpdate {
        ron::to_string(self).unwrap().into_bytes()
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SourceCode {
    pub fragments: Fragments,
    pub source_map: SourceMap,
}

pub type SerializedUpdate = Vec<u8>;
