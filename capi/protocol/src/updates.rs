use capi_compiler::{repr::fragments::Fragments, source_map::SourceMap};
use capi_process::{Bytecode, Host, Process};
use serde::{de::DeserializeOwned, Serialize};

use crate::memory::Memory;

pub struct Updates<H: Host> {
    latest_memory: Option<Memory>,
    process_at_client: Option<Process<H>>,
    queue: Vec<Update<H>>,
}

impl<H: Host> Updates<H> {
    pub fn queue_updates(&mut self, process: &Process<H>, memory: &Memory)
    where
        H: Clone + PartialEq,
    {
        self.latest_memory = Some(memory.clone());

        if self.update_is_necessary(process) {
            self.process_at_client = Some(process.clone());
            self.queue.push(Update::Process(process.clone()));

            if let Some(memory) = self.latest_memory.take() {
                self.queue.push(Update::Memory { memory });
            }
        }
    }

    pub fn take_queued_updates(
        &mut self,
    ) -> impl Iterator<Item = Update<H>> + '_ {
        self.queue.drain(..)
    }

    fn update_is_necessary(&self, process: &Process<H>) -> bool
    where
        H: PartialEq,
    {
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

impl<H: Host> Default for Updates<H> {
    fn default() -> Self {
        Self {
            latest_memory: Default::default(),
            process_at_client: Default::default(),
            queue: Default::default(),
        }
    }
}

#[allow(clippy::large_enum_variant)] // haven't optimized this yet
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Update<H: Host> {
    Process(Process<H>),
    Memory { memory: Memory },
}

impl<H: Host> Update<H> {
    pub fn deserialize(bytes: SerializedUpdate) -> Self
    where
        H: DeserializeOwned,
    {
        let string = std::str::from_utf8(&bytes).unwrap();
        ron::from_str(string).unwrap()
    }

    pub fn serialize(&self) -> SerializedUpdate
    where
        H: Serialize,
    {
        ron::to_string(self).unwrap().into_bytes()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Code {
    pub fragments: Fragments,
    pub bytecode: Bytecode,
    pub source_map: SourceMap,
}

pub type SerializedUpdate = Vec<u8>;
