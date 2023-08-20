use std::{
    path::{Path, PathBuf},
    sync::mpsc::{sync_channel, Receiver},
    time::Duration,
};

use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{
    DebounceEventResult, DebouncedEventKind, Debouncer,
};

use crate::loader::load::load;

pub fn watch(
    path: impl AsRef<Path>,
) -> anyhow::Result<(Receiver<String>, Debouncer<RecommendedWatcher>)> {
    let path_for_watcher = PathBuf::from(path.as_ref());

    let (sender, receiver) = sync_channel(0);

    let mut debouncer = notify_debouncer_mini::new_debouncer(
        Duration::from_millis(50),
        move |result: DebounceEventResult| {
            let path = &path_for_watcher;

            if let Ok(events) = result {
                for event in events {
                    if let DebouncedEventKind::Any = event.kind {
                        let code = load(path).unwrap();
                        sender.send(code).unwrap();
                    }
                }

                return;
            }

            // Not sure what else we can do about it here.
            eprintln!("Error watching code: {result:?}");
        },
    )?;

    debouncer
        .watcher()
        .watch(path.as_ref(), RecursiveMode::Recursive)?;

    Ok((receiver, debouncer))
}
