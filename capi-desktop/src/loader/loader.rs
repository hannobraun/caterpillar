use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use capi_core::{
    pipeline::{ScriptPath, Scripts},
    repr::eval::{fragments::FragmentId, value},
};
use notify::RecommendedWatcher;
use notify_debouncer_mini::Debouncer;
use walkdir::WalkDir;

use super::{channel::UpdateSender, watch::watch, Update, UpdateReceiver};

pub struct Loader {
    old_sender: UpdateSender,
    old_receiver: UpdateReceiver,
    receiver: UpdateReceiver,
    watchers: Vec<Debouncer<RecommendedWatcher>>,
    scripts: Scripts,
    update_available: bool,
}

impl Loader {
    pub fn new(entry_script_path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let entry_script_path = entry_script_path.into();

        let (old_sender, old_receiver) = crossbeam_channel::unbounded();
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut watchers = Vec::new();

        let mut entry_script_dir = entry_script_path.clone();
        entry_script_dir.pop();

        let mut scripts = BTreeMap::new();

        for entry in WalkDir::new(&entry_script_dir) {
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

        let scripts = scripts
            .into_iter()
            .map(|(path, code)| {
                let code = code.expect("Made sure scripts are complete");
                (path, code)
            })
            .collect();
        let scripts = Scripts { inner: scripts };

        // We just loaded the initial code, so if a caller asks us now to return
        // the current `Scripts`, we don't need that to wait for changes.
        //
        // By setting this flag, we signal that updates are currently available,
        // and make that happen.
        let update_available = true;

        Ok(Self {
            old_sender,
            old_receiver,
            receiver,
            watchers,
            scripts,
            update_available,
        })
    }

    pub fn wait_for_updated_scripts(&mut self) -> anyhow::Result<&Scripts> {
        if self.update_available {
            // An update is already available. We don't need to wait for the
            // next one, as the loop below would do.
            self.apply_available_update()?;
            self.update_available = false;
            return Ok(&self.scripts);
        }

        loop {
            let update = self.receiver.recv()?;
            handle_update(update, &mut self.scripts)?;

            if !self.receiver.is_empty() {
                continue;
            }

            break;
        }

        Ok(&self.scripts)
    }

    pub fn scripts_if_updated(&mut self) -> anyhow::Result<Option<&Scripts>> {
        self.apply_available_update()?;

        if self.update_available {
            self.update_available = false;
            Ok(Some(&self.scripts))
        } else {
            Ok(None)
        }
    }

    fn apply_available_update(&mut self) -> anyhow::Result<()> {
        for update in self.receiver.try_iter() {
            handle_update(update, &mut self.scripts)?;
            self.update_available = true;
        }

        Ok(())
    }

    /// This is a legacy method. It needs to be removed, once all callers have
    /// migrated to the new API.
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

    /// This is a legacy method. It needs to be removed, once all callers have
    /// migrated to the new API.
    pub fn updates(&self) -> &UpdateReceiver {
        &self.old_receiver
    }
}

fn handle_update(update: Update, scripts: &mut Scripts) -> anyhow::Result<()> {
    let (path, _, code) = update?;
    let path = fs_path_to_script_path(path);
    *scripts
        .inner
        .get_mut(&path)
        .expect("Receiving update for script; expected it to be known") = code;
    Ok(())
}

fn fs_path_to_script_path(path: PathBuf) -> ScriptPath {
    fs_path_to_symbols(path)
}

fn fs_path_to_symbols(path: impl AsRef<Path>) -> Vec<value::Symbol> {
    path.as_ref()
        .iter()
        .map(|os_str| {
            let string = os_str.to_string_lossy().into_owned();
            value::Symbol(string)
        })
        .collect()
}
