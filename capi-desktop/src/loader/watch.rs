use std::{path::PathBuf, time::Duration};

use crossbeam_channel::SendError;
use notify::RecursiveMode;
use notify_debouncer_mini::{DebounceEventResult, DebouncedEventKind};
use tracing::error;

use super::{script_loader::ScriptLoader, ScriptUpdates};

pub fn watch(path: PathBuf) -> anyhow::Result<ScriptUpdates> {
    let (sender, receiver) = crossbeam_channel::unbounded();
    let script_loader = ScriptLoader::new(path.clone(), sender)?;

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
