use std::{
    collections::VecDeque,
    future::{self, Future},
    path::{self, PathBuf},
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
    pub fn new(
        crates_dir: PathBuf,
        changes: mpsc::UnboundedReceiver<notify::Event>,
    ) -> Self {
        let (tx, rx) = watch::channel(());
        tokio::spawn(debounce(crates_dir, changes, tx));

        Self { changes: rx }
    }

    pub async fn wait_for_change(&mut self) -> bool {
        self.changes.changed().await.is_ok()
    }
}

async fn debounce(
    crates_dir: PathBuf,
    mut rx: mpsc::UnboundedReceiver<notify::Event>,
    tx: watch::Sender<()>,
) {
    let mut timers = VecDeque::new();

    loop {
        select! {
            event = rx.recv() => {
                let Some(event) = event else {
                    // The other end has hung up. This means we're done here
                    // too.
                    break;
                };

                for path in event.paths {
                    let path_within_crates_dir = path
                        .strip_prefix(&crates_dir)
                        .expect("Expected path within `crates/` directory");
                    let changed_crate = path_within_crates_dir
                        .components()
                        .next();

                    let Some(path::Component::Normal(changed_crate)) =
                        changed_crate
                    else {
                        // Whatever this is, it isn't a change to anything we're
                        // watching. I don't want to panic here though, because
                        // I expect that we might get weird paths like that, for
                        // example if we're moving something into or out of the
                        // watched directory.
                        continue;
                    };

                    // We have a change! Wait for a bit, before passing it on.
                    let timer = sleep(Duration::from_millis(20));
                    timers.push_back((
                        changed_crate.to_os_string(),
                        Box::pin(timer)
                    ));
                }
            }
            _ = SleepOption(timers.front_mut().map(|(_, timer)| timer)) => {
                let (changed_crate, _) = timers.pop_front()
                    .expect("Future was ready; must be `Some`");

                // We also need to throw away any changes that might or might
                // not have arrived in the meantime, or we haven't actually
                // debounced anything.
                timers.retain(|(c, _)| c != &changed_crate);

                if tx.send(()).is_err() {
                    // The other end has hung up. This means we're done here
                    // too.
                    break;
                }
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
