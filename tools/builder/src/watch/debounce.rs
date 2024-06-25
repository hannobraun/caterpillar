use std::time::Duration;

use tokio::{
    sync::{mpsc, watch},
    time::sleep,
};

#[derive(Clone)]
pub struct DebouncedChanges {
    changes: watch::Receiver<()>,
}

impl DebouncedChanges {
    pub fn new(changes: mpsc::UnboundedReceiver<notify::Event>) -> Self {
        let (tx, rx) = watch::channel(());
        tokio::spawn(debounce(changes, tx));

        Self { changes: rx }
    }

    pub async fn wait_for_change(&mut self) -> bool {
        self.changes.changed().await.is_ok()
    }
}

async fn debounce(
    mut rx: mpsc::UnboundedReceiver<notify::Event>,
    tx: watch::Sender<()>,
) {
    loop {
        if rx.recv().await.is_none() {
            // The other end has hung up. This means we're done here too.
            break;
        }

        // We have a change! Wait for a bit, before passing it on.
        sleep(Duration::from_millis(20)).await;
        if tx.send(()).is_err() {
            // The other end has hung up. This means we're done here too.
            break;
        }

        // We also need to throw away any changes that might or might not have
        // arrived in the meantime, or we haven't actually debounced anything.
        while rx.try_recv().is_ok() {}
    }
}
