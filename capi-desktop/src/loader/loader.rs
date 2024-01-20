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

// To adapt the `Loader` API for the ongoing changes to pipeline, we'll probably
// need something like the following changes:
//
// - Accept the path of the entry script in `new`, search for all scripts that
//   are reachable from that, and set up the watching/loading infrastructure for
//   all of them.
//   It will also initialize a `Scripts` instance and set everything up to keep
//   that updated.
// - No more `load`. It will no longer be necessary to load single scripts.
// - Instead of `updates`, have a method that provides access to the latest
//   version of `Scripts`.
//
// As a first iteration, I can probably reuse the watch/load infrastructure
// as-is. But if we're going to watch a whole directory tree (at least the
// `.capi` scripts in it), it'll probably end up simpler, if I adapt the
// infrastructure to that specific task.
//
// Maybe this change can be done in parallel to the existing API? As in, keep
// `load` and `updates` around while I build up the rest? Then I can use debug
// output to verify the new functionality works, while building it up step by
// step, never requiring a single huge change.
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
