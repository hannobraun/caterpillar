use std::{
    future::Future,
    path::Path,
    pin::{pin, Pin},
    task::{Context, Poll},
    time::Duration,
};

use notify::{RecursiveMode, Watcher as _};
use tokio::{
    sync::mpsc,
    time::{sleep, Sleep},
};
use tokio_stream::{wrappers::UnboundedReceiverStream, Stream};

pub fn watch() -> anyhow::Result<Watcher> {
    let (tx, rx) = mpsc::unbounded_channel();

    // We interpret the intent behind calling this function as wanting to "load"
    // the game code, as opposed to just wanting to watch it after possibly
    // having or not having loaded it via other means.
    //
    // Therefore, we need to trigger an initial change.
    tx.send(())?;

    let mut watcher = notify::recommended_watcher(move |_| {
        if tx.send(()).is_err() {
            // The other end has hung up. Not much we can do about that. The
            // thread this is running on will probably also end soon.
        }
    })?;
    watcher.watch(Path::new("capi"), RecursiveMode::Recursive)?;

    let changes = DebouncedChanges {
        changes: Changes::new(rx),
        delay: None,
    };

    Ok(Watcher {
        _watcher: watcher,
        changes,
    })
}

pub struct Watcher {
    _watcher: notify::RecommendedWatcher,
    pub changes: DebouncedChanges,
}

pub struct DebouncedChanges {
    changes: UnboundedReceiverStream<()>,
    delay: Option<Pin<Box<Sleep>>>,
}

impl Stream for DebouncedChanges {
    type Item = <Changes as Stream>::Item;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        // Start by processing all pending changes. To correctly debounce the
        // change stream, we need to do this first. Otherwise, if the delay is
        // over and we trigger a debounced change event during this call, any
        // unprocessed changes will trigger another debounced event, next time
        // we get polled.
        loop {
            match pin!(&mut self.changes).poll_next(cx) {
                Poll::Ready(Some(())) => {
                    if self.delay.is_none() {
                        // If there's a change, and there's no delay (meaning we
                        // have no debounced change in the pipeline yet), we
                        // start a delay, thereby triggering a debounced event.
                        //
                        // If there *is* a delay already, we don't need to do
                        // anything. The change we read just now just gets
                        // folded into the debounced event.
                        self.delay =
                            Some(Box::pin(sleep(Duration::from_millis(20))));
                    }
                }
                Poll::Ready(None) => {
                    // Looks like there are no more changes. Regardless of
                    // whatever else is going on, we're done here, forever.
                    return Poll::Ready(None);
                }
                Poll::Pending => {
                    // We processed all pending changes. Let's move on.
                    break;
                }
            }
        }

        // Okay, all pending changes got processed. Let's check if there's a
        // debounced change in the pipeline.
        if let Some(mut delay) = self.delay.take() {
            // There indeed is a debounced event in the pipeline. See if it's
            // ready.
            match delay.as_mut().poll(cx) {
                Poll::Ready(_) => {
                    // Debounced delay is ready! We are already consuming the
                    // delay here, so all that's left is to return the debounced
                    // change.
                    return Poll::Ready(Some(()));
                }
                Poll::Pending => {
                    // The debounced change is not ready yet. Put the delay back
                    // so we can check again later.
                    self.delay = Some(delay);
                }
            }
        }

        // If we made it here, that means nothing we looked at here amounted to
        // anything yet.
        Poll::Pending
    }
}

type Changes = UnboundedReceiverStream<()>;
