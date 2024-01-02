mod script_loader;

use std::{path::PathBuf, time::Duration};

use crossbeam_channel::{Receiver, SendError};
use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{
    DebounceEventResult, DebouncedEventKind, Debouncer,
};
use tracing::error;

use self::script_loader::ScriptLoader;

#[derive(Default)]
pub struct Loader {
    watchers: Vec<Debouncer<RecommendedWatcher>>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            watchers: Vec::new(),
        }
    }

    /// Load the script at the given path
    ///
    /// # Implementation Note
    ///
    /// It would be better to not return the code as a `String` here, and
    /// instead trigger the first update through the channel. Then we only have
    /// to support one code path for loading code.
    pub fn load(
        &mut self,
        path: impl Into<PathBuf>,
    ) -> anyhow::Result<(String, Receiver<anyhow::Result<String>>)> {
        let path = path.into();

        let ScriptUpdates {
            receiver: updates,
            watcher,
        } = watch(path)?;
        let code = updates.recv()??;

        self.watchers.push(watcher);
        Ok((code, updates))
    }
}

struct ScriptUpdates {
    pub receiver: Receiver<anyhow::Result<String>>,
    pub watcher: Debouncer<RecommendedWatcher>,
}

fn watch(path: PathBuf) -> anyhow::Result<ScriptUpdates> {
    let (script_loader, receiver) = ScriptLoader::new(path.clone())?;

    let mut debouncer = notify_debouncer_mini::new_debouncer(
        Duration::from_millis(50),
        move |result: DebounceEventResult| {
            let events = match result {
                Ok(events) => events,
                Err(err) => {
                    if let Err(SendError(err)) = script_loader.on_error(err) {
                        // If we end up here, the channel has been disconnected.
                        // Nothing we can do about it here, but log the error.
                        //
                        // If the channel is disconnected, the watcher should
                        // have been dropped too, which would mean this whole
                        // thread is about to die anyway.
                        error!("Failed to send code watching error: {err:?}");
                    }
                    return;
                }
            };

            for event in events {
                if let DebouncedEventKind::Any = event.kind {
                    if let Err(SendError(result)) = script_loader.trigger() {
                        // See comment above on why this is the appropriate way
                        // to handle this.
                        error!(
                            "Failed to send code loading result: {result:?}"
                        );
                    }
                }
            }
        },
    )?;

    debouncer
        .watcher()
        .watch(&path, RecursiveMode::NonRecursive)?;

    Ok(ScriptUpdates {
        receiver,
        watcher: debouncer,
    })
}
