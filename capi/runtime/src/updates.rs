use capi_compiler::{source_map::SourceMap, syntax};
use capi_process::Process;
use capi_protocol::{
    memory::Memory,
    update::{SerializedUpdate, Update},
};

pub struct Updates {
    latest_memory: Option<Memory>,
    process_at_client: Option<Process>,
    queue: Vec<SerializedUpdate>,
}

impl Updates {
    pub fn new() -> Self {
        Self {
            latest_memory: None,
            process_at_client: None,
            queue: Vec::new(),
        }
    }

    pub fn queue_source_code(
        &mut self,
        functions: syntax::Functions,
        source_map: SourceMap,
    ) {
        self.queue(Update::SourceCode {
            functions,
            source_map,
        });
    }

    pub fn queue_updates(&mut self, process: &Process, memory: &Memory) {
        self.latest_memory = Some(memory.clone());

        if self.update_is_necessary(process) {
            self.process_at_client = Some(process.clone());
            self.queue(Update::Process(process.clone()));

            if let Some(memory) = self.latest_memory.take() {
                self.queue(Update::Memory { memory });
            }
        }
    }

    pub fn take_queued_updates(
        &mut self,
    ) -> impl Iterator<Item = SerializedUpdate> + '_ {
        self.queue.drain(..)
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

    fn queue(&mut self, update: Update) {
        let update = update.serialize();
        self.queue.push(update);
    }
}
