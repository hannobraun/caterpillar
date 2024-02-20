use std::{collections::BTreeMap, time::Duration};

use async_channel::{Receiver, RecvError, Sender, TryRecvError};
use capi_core::{
    pipeline::Scripts,
    repr::eval::value,
    runtime::{evaluator::RuntimeState, interpreter::Interpreter},
};
use gloo_timers::future::sleep;
use tracing::debug;

use crate::platform::{Context, Event, Events, WebPlatform};

pub async fn run(
    script: &str,
    code: Receiver<String>,
    events: Sender<Event>,
) -> anyhow::Result<()> {
    debug!("Running script:\n{script}");

    let entry_script_path = vec![value::Symbol(String::from("default"))];
    let scripts = {
        let mut inner = BTreeMap::new();
        inner.insert(entry_script_path.clone(), String::new());

        Scripts {
            entry_script_path: entry_script_path.clone(),
            inner,
        }
    };

    let mut interpreter = Interpreter::<WebPlatform>::new(scripts)?;

    interpreter.update()?;

    let mut context = Context {
        events: Events { inner: events },
        sleep_duration: None,
    };

    let mut scripts_updated = false;

    loop {
        if scripts_updated {
            scripts_updated = false;
            if let Err(err) = interpreter.update() {
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
                    Ok(code) => {
                        *interpreter
                            .scripts()
                            .inner
                            .get_mut(&entry_script_path)
                            .expect("Code for entry script not found") = code;
                        scripts_updated = true;
                    }
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
                *interpreter
                    .scripts()
                    .inner
                    .get_mut(&entry_script_path)
                    .expect("Code for entry script not found") = code;
                scripts_updated = true;
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
