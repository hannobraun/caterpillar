use std::time::Duration;

use async_channel::{Receiver, Sender, TryRecvError};
use capi_core::{
    value, DataStackResult, FunctionState, Interpreter, PlatformFunction,
    RuntimeContext, RuntimeState,
};
use gloo_timers::future::sleep;
use tracing::debug;

pub async fn run(
    script: &str,
    code: Receiver<String>,
    events: Sender<Event>,
) -> anyhow::Result<()> {
    debug!("Running script:\n{script}");

    let mut interpreter = Interpreter::new(script)?;
    let mut context = Context {
        events: Events { inner: events },
        sleep_duration: None,
    };

    interpreter.register_platform([
        ("delay_ms", delay_ms as PlatformFunction<Context>),
        ("print", print),
    ]);

    let mut new_code = None;

    loop {
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

        if let Some(code) = new_code.take() {
            interpreter.update(&code)?;
        }

        let sleep_duration = match interpreter.step(&mut context)? {
            RuntimeState::Running => None,
            RuntimeState::Sleeping => context.sleep_duration.take(),
            RuntimeState::Finished => break,
        };

        // Always sleep, even if it's for zero duration, to give the rest of the
        // website a chance to do its thing between steps.
        let sleep_duration = sleep_duration.unwrap_or(Duration::from_millis(0));
        sleep(sleep_duration).await;
    }

    Ok(())
}

pub struct Context {
    pub events: Events,
    pub sleep_duration: Option<Duration>,
}

pub struct Events {
    pub inner: Sender<Event>,
}

impl Events {
    pub fn output(&self, message: String) {
        self.inner.send_blocking(Event::Output(message)).unwrap()
    }
}

pub enum Event {
    Output(String),
}

pub fn delay_ms(
    runtime_context: RuntimeContext,
    platform_context: &mut Context,
) -> DataStackResult<FunctionState> {
    let (delay_ms, _) =
        runtime_context.data_stack.pop_specific::<value::Number>()?;

    let delay_ms = delay_ms
        .0
        .try_into()
        .expect("Negative sleep duration is invalid");
    platform_context.sleep_duration = Some(Duration::from_millis(delay_ms));

    Ok(FunctionState::Sleeping)
}

pub fn print(
    runtime_context: RuntimeContext,
    platform_context: &mut Context,
) -> DataStackResult<FunctionState> {
    let value = runtime_context.data_stack.pop_any()?;
    platform_context
        .events
        .output(format!("{}\n", value.payload));
    runtime_context.data_stack.push(value);
    Ok(FunctionState::Done)
}
