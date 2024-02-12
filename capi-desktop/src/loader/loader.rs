use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};
use capi_core::{
    pipeline::{ScriptPath, Scripts},
    repr::eval::value,
};
use notify::RecommendedWatcher;
use notify_debouncer_mini::Debouncer;
use walkdir::WalkDir;

use super::{channel::UpdateSender, watch::watch, Update, UpdateReceiver};

pub struct Loader {
    receiver: UpdateReceiver,
    _watchers: Vec<Debouncer<RecommendedWatcher>>,
    entry_script_dir: PathBuf,
    update_available: bool,
}

impl Loader {
    pub fn new(
        entry_script_path: impl Into<PathBuf>,
    ) -> anyhow::Result<(Self, Scripts)> {
        let entry_script_path = entry_script_path.into();

        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut watchers = Vec::new();

        let entry_script_dir = entry_script_path
            .canonicalize()?
            .parent()
            .ok_or_else(|| {
                anyhow!(
                    "Invalid entry script path `{}`; no parent",
                    entry_script_path.display()
                )
            })?
            .to_path_buf();

        let mut scripts = BTreeMap::new();

        walk_entry_script_dir(
            &entry_script_dir,
            &sender,
            &mut watchers,
            &mut scripts,
        )
        .with_context(|| {
            format!(
                "Error while walking entry script directory `{}`",
                entry_script_dir.display()
            )
        })?;

        loop {
            let all_scripts_loaded =
                scripts.values().all(|code| code.is_some());
            if all_scripts_loaded {
                break;
            }

            let (path, _, code) = receiver.recv()??;
            let path = fs_path_to_script_path(&entry_script_dir, path)?;
            scripts.insert(path, Some(code));
        }

        let scripts = scripts
            .into_iter()
            .map(|(path, code)| {
                let code = code.expect("Made sure scripts are complete");
                (path, code)
            })
            .collect();
        let scripts = Scripts {
            entry_script_path: fs_path_to_script_path(
                &entry_script_dir,
                entry_script_path,
            )?,
            inner: scripts,
        };

        // We just loaded the initial code, so if a caller asks us now to return
        // the current `Scripts`, we don't need that to wait for changes.
        //
        // By setting this flag, we signal that updates are currently available,
        // and make that happen.
        let update_available = true;

        Ok((
            Self {
                receiver,
                _watchers: watchers,
                entry_script_dir,
                update_available,
            },
            scripts,
        ))
    }

    pub fn wait_for_updates(
        &mut self,
    ) -> anyhow::Result<impl Iterator<Item = (ScriptPath, String)>> {
        let updates = if self.receiver.is_empty() {
            // If there are no updates, we need to wait for the next one.
            //
            // There's a race condition between the checking we just did, and
            // the waiting we're about to do. Doesn't matter though. The worst
            // that can happen, is that an update arrived in the meantime, and
            // our wait is very short.

            vec![self.receiver.recv()?]
        } else {
            // If there are updates, we can return them.
            //
            // Again, there's a race condition between the check above and our
            // upcoming collection of the updates. And again, it doesn't matter.
            // The worst that can happen is that more updates are available, and
            // then we just collect those too.

            self.receiver.try_iter().collect::<Vec<_>>()
        };

        convert_updates(updates, &self.entry_script_dir)
    }

    pub fn apply_update_if_available(
        &mut self,
        scripts: &mut Scripts,
    ) -> anyhow::Result<bool> {
        self.apply_available_update(scripts)?;

        if self.update_available {
            self.update_available = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn apply_available_update(
        &mut self,
        scripts: &mut Scripts,
    ) -> anyhow::Result<()> {
        for update in self.receiver.try_iter() {
            handle_update(update, &self.entry_script_dir, scripts)?;
            self.update_available = true;
        }

        Ok(())
    }
}

fn walk_entry_script_dir(
    entry_script_dir: impl AsRef<Path>,
    sender: &UpdateSender,
    watchers: &mut Vec<Debouncer<RecommendedWatcher>>,
    scripts: &mut BTreeMap<ScriptPath, Option<String>>,
) -> anyhow::Result<()> {
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

        let path = fs_path_to_script_path(&entry_script_dir, path)?;
        scripts.insert(path, None);
    }

    Ok(())
}

fn convert_updates(
    updates: impl IntoIterator<Item = Update>,
    entry_script_dir: &Path,
) -> anyhow::Result<impl Iterator<Item = (ScriptPath, String)>> {
    let updates = updates
        .into_iter()
        .map(|result| {
            result.and_then(|(path, _, code)| {
                fs_path_to_script_path(entry_script_dir, path)
                    .map(|path| (path, code))
            })
        })
        .collect::<anyhow::Result<Vec<(ScriptPath, String)>>>()?
        .into_iter();

    Ok(updates)
}

fn handle_update(
    update: Update,
    entry_script_dir: impl AsRef<Path>,
    scripts: &mut Scripts,
) -> anyhow::Result<()> {
    let (path, _, code) = update?;
    let path = fs_path_to_script_path(entry_script_dir, path)?;
    *scripts
        .inner
        .get_mut(&path)
        .expect("Receiving update for script; expected it to be known") = code;
    Ok(())
}

fn fs_path_to_script_path(
    entry_script_dir: impl AsRef<Path>,
    path: PathBuf,
) -> anyhow::Result<ScriptPath> {
    let entry_script_dir = entry_script_dir.as_ref().canonicalize()?;
    let path = path.canonicalize()?;

    let mut entry_script_dir_symbols = fs_path_to_symbols(entry_script_dir);
    let mut script_path_symbols = fs_path_to_symbols(path);

    loop {
        let first_entry_script_dir_symbol = entry_script_dir_symbols.first();
        let first_script_path_symbol = script_path_symbols.first();

        if let (
            Some(first_entry_script_dir_symbol),
            Some(first_script_path_symbol),
        ) = (first_entry_script_dir_symbol, first_script_path_symbol)
        {
            if first_entry_script_dir_symbol == first_script_path_symbol {
                entry_script_dir_symbols.remove(0);
                script_path_symbols.remove(0);

                continue;
            }
        }

        break;
    }

    if let Some(file_name) = script_path_symbols.last_mut() {
        if let Some(index) = file_name.0.rfind('.') {
            file_name.0.truncate(index);
        }
    }

    Ok(script_path_symbols)
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
