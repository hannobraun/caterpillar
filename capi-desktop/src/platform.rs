use std::{thread, time::Duration};

use capi_core::{
    platform::Platform, value, DataStackResult, PlatformFunction,
    PlatformFunctionState, RuntimeContext,
};

pub struct DesktopPlatform;

impl Platform for DesktopPlatform {
    type Context = PlatformContext;

    fn functions(
    ) -> impl IntoIterator<Item = (PlatformFunction<PlatformContext>, &'static str)>
    {
        [
            (
                clear_pixel as PlatformFunction<PlatformContext>,
                "clear_pixel",
            ),
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
    #[allow(clippy::new_without_default)]
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
    runtime_context: RuntimeContext,
    platform_context: &mut PlatformContext,
) -> DataStackResult<PlatformFunctionState> {
    let (y, _) = runtime_context.data_stack.pop_specific::<value::Number>()?;
    let (x, _) = runtime_context.data_stack.pop_specific::<value::Number>()?;

    platform_context.pixel_ops.send(PixelOp::Clear([x.0, y.0]));

    Ok(PlatformFunctionState::Done)
}

fn delay_ms(
    runtime_context: RuntimeContext,
    _: &mut PlatformContext,
) -> DataStackResult<PlatformFunctionState> {
    let (delay_ms, _) =
        runtime_context.data_stack.pop_specific::<value::Number>()?;
    thread::sleep(Duration::from_millis(delay_ms.0.try_into().unwrap()));
    Ok(PlatformFunctionState::Done)
}

fn print(
    runtime_context: RuntimeContext,
    _: &mut PlatformContext,
) -> DataStackResult<PlatformFunctionState> {
    let value = runtime_context.data_stack.pop_any()?;
    println!("{}", value.payload);
    runtime_context.data_stack.push(value);
    Ok(PlatformFunctionState::Done)
}

fn set_pixel(
    runtime_context: RuntimeContext,
    platform_context: &mut PlatformContext,
) -> DataStackResult<PlatformFunctionState> {
    let (y, _) = runtime_context.data_stack.pop_specific::<value::Number>()?;
    let (x, _) = runtime_context.data_stack.pop_specific::<value::Number>()?;

    platform_context.pixel_ops.send(PixelOp::Set([x.0, y.0]));

    Ok(PlatformFunctionState::Done)
}
