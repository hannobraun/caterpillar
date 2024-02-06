use std::time::Duration;

use async_channel::Sender;
use capi_core::{
    platform::{
        BuiltinFn, BuiltinFnResult, BuiltinFnState, BuiltinFns, CoreContext,
        Platform,
    },
    repr::eval::value,
};
use chrono::Local;
use futures::executor::block_on;

pub struct WebPlatform;

impl Platform for WebPlatform {
    type Context<'r> = Context;
    type Error = ();

    fn builtin_fns() -> impl BuiltinFns<Self> {
        [(delay_ms as BuiltinFn<Self>, "delay_ms"), (print, "print")]
    }
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
        block_on(self.inner.send(Event::Output(message))).unwrap()
    }

    pub fn status(&self, message: impl Into<String>) {
        let message = format!(
            "{}: {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            message.into()
        );
        block_on(self.inner.send(Event::Status(message))).unwrap()
    }
}

pub enum Event {
    Output(String),
    Status(String),
}

pub fn delay_ms(
    step: usize,
    runtime_context: CoreContext,
    platform_context: &mut Context,
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (delay_ms, _) =
                runtime_context.data_stack.pop_specific::<value::Number>()?;

            let delay_ms = delay_ms
                .0
                .try_into()
                .expect("Negative sleep duration is invalid");
            platform_context.sleep_duration =
                Some(Duration::from_millis(delay_ms));

            Ok(BuiltinFnState::Sleeping)
        }
        _ => unreachable!(),
    }
}

pub fn print(
    step: usize,
    runtime_context: CoreContext,
    platform_context: &mut Context,
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let value = runtime_context.data_stack.pop_any()?;
            platform_context
                .events
                .output(format!("{}\n", value.payload));
            runtime_context.data_stack.push(value);
            Ok(BuiltinFnState::Completed)
        }
        _ => unreachable!(),
    }
}
