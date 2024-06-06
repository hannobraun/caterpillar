use capi_compiler::{fragments::Fragments, source_map::SourceMap};
use capi_game_engine::memory::Memory;
use capi_runtime::{Instructions, Runtime};

#[derive(Debug, Default)]
pub struct Updates {
    latest_memory: Option<Memory>,
    runtime_at_client: Option<Runtime>,
    queue: Vec<UpdateFromHost>,
}

impl Updates {
    pub fn queue_updates(&mut self, process: &Runtime, memory: &Memory) {
        self.latest_memory = Some(memory.clone());

        if self.update_is_necessary(process) {
            self.runtime_at_client = Some(process.clone());
            self.queue.push(UpdateFromHost::Process(process.clone()));

            if let Some(memory) = self.latest_memory.take() {
                self.queue.push(UpdateFromHost::Memory { memory });
            }
        }
    }

    pub fn take_queued_updates(
        &mut self,
    ) -> impl Iterator<Item = UpdateFromHost> + '_ {
        self.queue.drain(..)
    }

    fn update_is_necessary(&self, process: &Runtime) -> bool {
        if let Some(process_at_client) = &self.runtime_at_client {
            // The client has previously received a program. We don't want to
            // saturate the connection with useless updates, so use that to
            // determine, if we should send an update.

            if process_at_client.state().is_running()
                && process.state().is_running()
            {
                // While the program is running, sending updates on every change
                // would result in too many updates.
                return false;
            }
        }

        self.runtime_at_client.as_ref() != Some(process)
    }
}

#[allow(clippy::large_enum_variant)] // haven't optimized this yet
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum UpdateFromHost {
    Process(Runtime),
    Memory { memory: Memory },
}

impl UpdateFromHost {
    pub fn deserialize(bytes: SerializedUpdate) -> Self {
        let string = std::str::from_utf8(&bytes).unwrap();
        ron::from_str(string).unwrap()
    }

    pub fn serialize(&self) -> SerializedUpdate {
        ron::to_string(self).unwrap().into_bytes()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Code {
    pub fragments: Fragments,
    pub instructions: Instructions,
    pub source_map: SourceMap,
}

pub type SerializedUpdate = Vec<u8>;
