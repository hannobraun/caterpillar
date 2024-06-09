use std::{
    pin::{pin, Pin},
    task::{Context, Poll},
};

use tokio::sync::mpsc;
use tokio_stream::Stream;

use super::raw::Changes;

pub struct FilteredChanges {
    pub changes: Changes,
}

impl FilteredChanges {
    pub fn new(changes: mpsc::UnboundedReceiver<()>) -> Self {
        Self {
            changes: Changes::new(changes),
        }
    }
}

impl Stream for FilteredChanges {
    type Item = <Changes as Stream>::Item;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        pin!(&mut self.changes).poll_next(cx)
    }
}
