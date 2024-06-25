use std::{
    collections::VecDeque,
    future::{self, Future},
    pin::{pin, Pin},
    task::{Context, Poll},
    time::Duration,
};

use tokio::{
    select,
    sync::{mpsc, watch},
    time::{sleep, Sleep},
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
    let mut timers = VecDeque::new();

    loop {
        select! {
            event = rx.recv() => {
                if event.is_none() {
                    // The other end has hung up. This means we're done here
                    // too.
                    break;
                }

                // We have a change! Wait for a bit, before passing it on.
                let timer = sleep(Duration::from_millis(20));
                timers.push_back(Box::pin(timer));
            }
            _ = SleepOption(timers.front_mut()) => {
                timers.pop_front();

                if tx.send(()).is_err() {
                    // The other end has hung up. This means we're done here
                    // too.
                    break;
                }

                // We also need to throw away any changes that might or might
                // not have arrived in the meantime, or we haven't actually
                // debounced anything.
                timers.clear();
            }
        }
    }
}

pub struct SleepOption<'r>(Option<&'r mut Pin<Box<Sleep>>>);

impl Future for SleepOption<'_> {
    type Output = <Sleep as Future>::Output;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        match &mut self.0 {
            Some(sleep) => sleep.as_mut().poll(cx),
            None => {
                let mut pending = pin!(future::pending());
                pending.as_mut().poll(cx)
            }
        }
    }
}
