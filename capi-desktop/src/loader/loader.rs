use std::path::PathBuf;

use capi_core::repr::eval::fragments::FragmentId;
use notify::RecommendedWatcher;
use notify_debouncer_mini::Debouncer;

use super::{channel::UpdateSender, watch::watch, UpdateReceiver};

pub struct Loader {
    sender: UpdateSender,
    receiver: UpdateReceiver,
    watchers: Vec<Debouncer<RecommendedWatcher>>,
}

impl Loader {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();

        Self {
            sender,
            receiver,
            watchers: Vec::new(),
        }
    }

    pub fn load(
        &mut self,
        path: impl Into<PathBuf>,
        parent: Option<FragmentId>,
    ) -> anyhow::Result<()> {
        let path = path.into();

        let watcher = watch(path, parent, self.sender.clone())?;
        self.watchers.push(watcher);

        Ok(())
    }

    pub fn updates(&self) -> &UpdateReceiver {
        &self.receiver
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::new()
    }
}
