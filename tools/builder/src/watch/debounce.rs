use std::{
    future::Future,
    pin::{pin, Pin},
    task::{Context, Poll},
    time::Duration,
};

use tokio::{
    sync::mpsc,
    time::{sleep, Sleep},
};
use tokio_stream::{Stream, StreamExt};

use super::raw::RawChanges;

pub struct DebouncedChanges {
    changes: RawChanges,
    delay: Option<Pin<Box<Sleep>>>,
}

impl DebouncedChanges {
    pub fn new(changes: mpsc::UnboundedReceiver<()>) -> Self {
        Self {
            changes: RawChanges::new(changes),
            delay: None,
        }
    }

    pub async fn wait_for_change(&mut self) -> bool {
        self.next().await.is_some()
    }
}

impl Stream for DebouncedChanges {
    type Item = <RawChanges as Stream>::Item;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        // Start by processing all pending changes. To correctly debounce the
        // change stream, we need to do this first. Otherwise, if the delay is
        // over and we trigger a debounced change event during this call, any
        // unprocessed changes will trigger another debounced event, next time
        // we get polled.
        let mut change = false;
        loop {
            match pin!(&mut self.changes).poll_next(cx) {
                Poll::Ready(Some(())) => {
                    change = true;
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

        if change && self.delay.is_none() {
            // If there's a change, and there's no delay (meaning we have no
            // debounced change in the pipeline yet), we start a delay, thereby
            // triggering a debounced event.
            //
            // If there *is* a delay already, we don't need to do anything. The
            // change we read just now gets folded into the debounced event.
            self.delay = Some(Box::pin(sleep(Duration::from_millis(20))));
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
