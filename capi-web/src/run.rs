use std::time::Duration;

use async_channel::{Receiver, RecvError, Sender, TryRecvError};
use capi_core::{pipeline::Scripts, Interpreter, RuntimeState};
use gloo_timers::future::sleep;
use tracing::debug;

use crate::platform::{Context, Event, Events, WebPlatform};

pub async fn run(
    script: &str,
    code: Receiver<String>,
    events: Sender<Event>,
) -> anyhow::Result<()> {
    debug!("Running script:\n{script}");

    let scripts = Scripts::default();
    let mut interpreter = Interpreter::<WebPlatform>::new()?;

    let parent = None;
    interpreter.update(parent, &scripts)?;

    let mut context = Context {
        events: Events { inner: events },
        sleep_duration: None,
    };

    let mut new_code: Option<String> = None;

    loop {
        if new_code.take().is_some() {
            let parent = None;
            if let Err(err) = interpreter.update(parent, &scripts) {
                context.events.status(format!("Pipeline error:\n{err:?}\n"));
            }
        }

        let sleep_duration = match interpreter.step(&mut context) {
            Ok(RuntimeState::Running) => None,
            Ok(RuntimeState::Sleeping) => context.sleep_duration.take(),
            Ok(RuntimeState::Finished) => {
                context.events.status(
                    "Program finished (will restart on change to script)\n",
                );

                match code.recv().await {
                    Ok(code) => new_code = Some(code),
                    Err(RecvError) => {
                        // The channel was closed. However this happened, it
                        // means our work here is done.
                        break;
                    }
                }

                context.events.status("Change detected. Restarting.\n");

                continue;
            }
            Err(err) => {
                context.events.status(format!("Runtime error:\n{err:?}\n"));
                break;
            }
        };

        // Always sleep, even if it's for zero duration, to give the rest of the
        // website a chance to do its thing between steps.
        let sleep_duration = sleep_duration.unwrap_or(Duration::from_millis(0));
        sleep(sleep_duration).await;

        match code.try_recv() {
            Ok(code) => {
                new_code = Some(code);
            }
            Err(TryRecvError::Empty) => {
                // No problem that we don't have a code update. Just continue.
            }
            Err(TryRecvError::Closed) => {
                // The channel was closed. However this happened, it means our
                // work here is done.
                break;
            }
        }
    }

    Ok(())
}
