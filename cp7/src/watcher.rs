use std::{path::Path, time::Duration};

use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{
    DebounceEventResult, DebouncedEventKind, Debouncer,
};

pub fn watch(
    path: impl AsRef<Path>,
) -> anyhow::Result<Debouncer<RecommendedWatcher>> {
    let mut debouncer = notify_debouncer_mini::new_debouncer(
        Duration::from_millis(50),
        None,
        |result: DebounceEventResult| {
            if let Ok(events) = result {
                for event in events {
                    if let DebouncedEventKind::Any = event.kind {
                        dbg!(event);
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

    Ok(debouncer)
}
