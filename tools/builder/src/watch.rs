use std::{
    path::Path,
    pin::{pin, Pin},
    task::{Context, Poll},
};

use notify::{RecursiveMode, Watcher as _};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::UnboundedReceiverStream, Stream};

pub fn watch() -> anyhow::Result<Watcher> {
    let (tx, rx) = mpsc::unbounded_channel();

    let mut watcher = notify::recommended_watcher(move |_| {
        if tx.send(()).is_err() {
            // The other end has hung up. Not much we can do about that. The
            // thread this is running on will probably also end soon.
        }
    })?;
    watcher.watch(Path::new("capi"), RecursiveMode::Recursive)?;

    let changes = Changes::new(rx);

    Ok(Watcher {
        _watcher: watcher,
        changes: DebouncedChanges { inner: changes },
    })
}

pub struct Watcher {
    _watcher: notify::RecommendedWatcher,
    pub changes: DebouncedChanges,
}

pub struct DebouncedChanges {
    inner: UnboundedReceiverStream<()>,
}

impl Stream for DebouncedChanges {
    type Item = <Changes as Stream>::Item;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        pin!(&mut self.inner).poll_next(cx)
    }
}

pub type Changes = UnboundedReceiverStream<()>;
