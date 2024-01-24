use std::{collections::BTreeMap, path::PathBuf};

use capi_core::{
    pipeline::ScriptPath,
    repr::eval::{fragments::FragmentId, value},
};
use notify::RecommendedWatcher;
use notify_debouncer_mini::Debouncer;
use walkdir::WalkDir;

use super::{channel::UpdateSender, watch::watch, UpdateReceiver};

pub struct Loader {
    old_sender: UpdateSender,
    old_receiver: UpdateReceiver,
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
    pub fn new(entry_script_path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let entry_script_path = entry_script_path.into();

        let (old_sender, old_receiver) = crossbeam_channel::unbounded();
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut watchers = Vec::new();

        let mut entry_script_dir = entry_script_path.clone();
        entry_script_dir.pop();

        let mut scripts = BTreeMap::new();

        for entry in WalkDir::new(entry_script_dir) {
            let entry = entry?;

            if entry.file_type().is_dir() {
                continue;
            }

            if !entry.file_name().as_encoded_bytes().ends_with(b".capi") {
                continue;
            }

            let path = entry.path().to_path_buf();

            let watcher = watch(path.clone(), None, sender.clone())?;
            watchers.push(watcher);

            let path = fs_path_to_script_path(path);
            scripts.insert(path, None);
        }

        loop {
            let all_scripts_loaded =
                scripts.values().all(|code| code.is_some());
            if all_scripts_loaded {
                break;
            }

            let (path, _, code) = receiver.recv()??;
            let path = fs_path_to_script_path(path);
            scripts.insert(path, Some(code));
        }

        dbg!(scripts);

        Ok(Self {
            old_sender,
            old_receiver,
            receiver,
            watchers,
        })
    }

    pub fn scripts_if_changed(&mut self) -> anyhow::Result<()> {
        for update in self.receiver.try_iter() {
            let (path, _, _) = update?;

            let path = path
                .iter()
                .map(|os_str| {
                    let string = os_str.to_string_lossy().into_owned();
                    value::Symbol(string)
                })
                .collect::<Vec<_>>();

            dbg!(path);
        }

        Ok(())
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

fn fs_path_to_script_path(path: PathBuf) -> ScriptPath {
    path.iter()
        .map(|os_str| {
            let string = os_str.to_string_lossy().into_owned();
            value::Symbol(string)
        })
        .collect::<Vec<_>>()
}
