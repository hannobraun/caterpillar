use std::{thread, time::Duration};

use capi_core::{
    value, DataStackResult, FunctionState, Interpreter, PlatformFunction,
    RuntimeContext,
};

pub struct Context {
    pub pixel_ops: Sender,
}

pub struct Sender {
    pub inner: crossbeam_channel::Sender<PixelOp>,
}

impl Sender {
    pub fn send(&self, message: PixelOp) {
        // Can return an error, if the channel is disconnected. This regularly
        // happens on shutdown, so let's just ignore it.
        let _ = self.inner.send(message);
    }
}

pub enum PixelOp {
    Clear([i64; 2]),
    Set([i64; 2]),
}

pub fn register(interpreter: &mut Interpreter<Context>) {
    interpreter.register_platform([
        (clear_pixel as PlatformFunction<Context>, "clear_pixel"),
        (delay_ms, "delay_ms"),
        (set_pixel, "set_pixel"),
        (print, "print"),
    ]);
}

fn clear_pixel(
    runtime_context: RuntimeContext,
    platform_context: &mut Context,
) -> DataStackResult<FunctionState> {
    let (y, _) = runtime_context.data_stack.pop_specific::<value::Number>()?;
    let (x, _) = runtime_context.data_stack.pop_specific::<value::Number>()?;

    platform_context.pixel_ops.send(PixelOp::Clear([x.0, y.0]));

    Ok(FunctionState::Done)
}

fn delay_ms(
    runtime_context: RuntimeContext,
    _: &mut Context,
) -> DataStackResult<FunctionState> {
    let (delay_ms, _) =
        runtime_context.data_stack.pop_specific::<value::Number>()?;
    thread::sleep(Duration::from_millis(delay_ms.0.try_into().unwrap()));
    Ok(FunctionState::Done)
}

fn print(
    runtime_context: RuntimeContext,
    _: &mut Context,
) -> DataStackResult<FunctionState> {
    let value = runtime_context.data_stack.pop_any()?;
    println!("{}", value.payload);
    runtime_context.data_stack.push(value);
    Ok(FunctionState::Done)
}

fn set_pixel(
    runtime_context: RuntimeContext,
    platform_context: &mut Context,
) -> DataStackResult<FunctionState> {
    let (y, _) = runtime_context.data_stack.pop_specific::<value::Number>()?;
    let (x, _) = runtime_context.data_stack.pop_specific::<value::Number>()?;

    platform_context.pixel_ops.send(PixelOp::Set([x.0, y.0]));

    Ok(FunctionState::Done)
}
