use std::path::PathBuf;

use capi_core::repr::eval::fragments::FragmentId;
use notify::RecommendedWatcher;
use notify_debouncer_mini::Debouncer;

use super::{channel::UpdateSender, watch::watch, UpdateReceiver};

pub struct Loader {
    old_sender: UpdateSender,
    old_receiver: UpdateReceiver,
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
// It would probably be best to build up the new API in parallel to the existing
// one. Then I can build the new API incrementally, test my work with debug
// output, and there won't have to be a huge change to switch everything over at
// once.
//
// If I'm going to reuse the existing watch/load infrastructure for that (which
// would probably be good in the short term, but longer-term, watching a whole
// directory tree would work better with a different architecture), I have to be
// careful not to interfere with the current workings. I can run the same code,
// but I can't use the same channel.
impl Loader {
    pub fn new(_entry_script_path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let (old_sender, old_receiver) = crossbeam_channel::unbounded();

        Ok(Self {
            old_sender,
            old_receiver,
            watchers: Vec::new(),
        })
    }

    pub fn load(
        &mut self,
        path: impl Into<PathBuf>,
        parent: Option<FragmentId>,
    ) -> anyhow::Result<()> {
        let path = path.into();

        let watcher = watch(path, parent, self.old_sender.clone())?;
        self.watchers.push(watcher);

        Ok(())
    }

    pub fn updates(&self) -> &UpdateReceiver {
        &self.old_receiver
    }
}
