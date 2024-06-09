use std::{
    pin::{pin, Pin},
    task::{Context, Poll},
};

use tokio::sync::mpsc;
use tokio_stream::Stream;

use super::raw::RawChanges;

pub struct FilteredChanges {
    pub changes: RawChanges,
}

impl FilteredChanges {
    pub fn new(changes: mpsc::UnboundedReceiver<()>) -> Self {
        Self {
            changes: RawChanges::new(changes),
        }
    }
}

impl Stream for FilteredChanges {
    type Item = ();

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        pin!(&mut self.changes).poll_next(cx)
    }
}
