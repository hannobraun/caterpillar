use std::{thread, time::Duration};

use capi_core::{
    platform::{BuiltinFn, BuiltinFnState, BuiltinFns, CoreContext, Platform},
    repr::eval::value,
    runtime::data_stack::DataStackResult,
};

pub struct DesktopPlatform;

impl Platform for DesktopPlatform {
    type Context = PlatformContext;

    fn builtin_fns() -> impl BuiltinFns<Self> {
        [
            (clear_pixel as BuiltinFn<Self>, "clear_pixel"),
            (delay_ms, "delay_ms"),
            (print, "print"),
            (set_pixel, "set_pixel"),
        ]
    }
}

pub struct PlatformContext {
    pub pixel_ops: Sender,
}

impl PlatformContext {
    pub fn new() -> Self {
        let (pixel_ops, _) = crossbeam_channel::unbounded();

        Self {
            pixel_ops: Sender { inner: pixel_ops },
        }
    }

    pub fn with_pixel_ops_sender(
        mut self,
        pixel_ops: crossbeam_channel::Sender<PixelOp>,
    ) -> Self {
        self.pixel_ops.inner = pixel_ops;
        self
    }
}

impl Default for PlatformContext {
    fn default() -> Self {
        Self::new()
    }
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

fn clear_pixel(
    step: usize,
    runtime_context: CoreContext,
    platform_context: &mut PlatformContext,
) -> DataStackResult<BuiltinFnState> {
    match step {
        0 => {
            let (y, _) =
                runtime_context.data_stack.pop_specific::<value::Number>()?;
            let (x, _) =
                runtime_context.data_stack.pop_specific::<value::Number>()?;

            platform_context.pixel_ops.send(PixelOp::Clear([x.0, y.0]));

            Ok(BuiltinFnState::Done)
        }
        _ => unreachable!(),
    }
}

fn delay_ms(
    step: usize,
    runtime_context: CoreContext,
    _: &mut PlatformContext,
) -> DataStackResult<BuiltinFnState> {
    match step {
        0 => {
            let (delay_ms, _) =
                runtime_context.data_stack.pop_specific::<value::Number>()?;
            thread::sleep(Duration::from_millis(
                delay_ms.0.try_into().unwrap(),
            ));
            Ok(BuiltinFnState::Done)
        }
        _ => unreachable!(),
    }
}

fn print(
    step: usize,
    runtime_context: CoreContext,
    _: &mut PlatformContext,
) -> DataStackResult<BuiltinFnState> {
    match step {
        0 => {
            let value = runtime_context.data_stack.pop_any()?;
            println!("{}", value.payload);
            runtime_context.data_stack.push(value);
            Ok(BuiltinFnState::Done)
        }
        _ => unreachable!(),
    }
}

fn set_pixel(
    step: usize,
    runtime_context: CoreContext,
    platform_context: &mut PlatformContext,
) -> DataStackResult<BuiltinFnState> {
    match step {
        0 => {
            let (y, _) =
                runtime_context.data_stack.pop_specific::<value::Number>()?;
            let (x, _) =
                runtime_context.data_stack.pop_specific::<value::Number>()?;

            platform_context.pixel_ops.send(PixelOp::Set([x.0, y.0]));

            Ok(BuiltinFnState::Done)
        }
        _ => unreachable!(),
    }
}
