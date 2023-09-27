use std::time::Duration;

use capi_core::{value, DataStackResult, FunctionState, RuntimeContext};

pub async fn run(script: &str) -> anyhow::Result<()> {
    tracing::debug!("Running script:\n{script}");

    let mut interpreter = capi_core::Interpreter::new(script)?;
    let mut context = Context::default();

    interpreter.register_platform([
        ("delay_ms", delay_ms as capi_core::PlatformFunction<Context>),
        ("print", print),
    ]);

    loop {
        let sleep_duration = match interpreter.step(&mut context)? {
            capi_core::RuntimeState::Running => None,
            capi_core::RuntimeState::Sleeping => context.sleep_duration.take(),
            capi_core::RuntimeState::Finished => break,
        };

        // Always sleep, even if it's for zero duration, to give the rest of the
        // website a chance to do its thing between steps.
        let sleep_duration =
            sleep_duration.unwrap_or(std::time::Duration::from_millis(0));
        gloo_timers::future::sleep(sleep_duration).await
    }

    Ok(())
}

#[derive(Default)]
pub struct Context {
    pub sleep_duration: Option<Duration>,
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
    _: &mut Context,
) -> DataStackResult<FunctionState> {
    let value = runtime_context.data_stack.pop_any()?;
    tracing::info!("{}", value.kind);
    runtime_context.data_stack.push(value);
    Ok(FunctionState::Done)
}
