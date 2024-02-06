use std::{thread, time::Duration};

use capi_core::{
    platform::{
        BuiltinFn, BuiltinFnResult, BuiltinFnState, BuiltinFns, CoreContext,
        Platform,
    },
    repr::eval::value,
};

pub struct DesktopPlatform;

impl Platform for DesktopPlatform {
    type Context<'r> = PlatformContext<'r>;
    type Error = ();

    fn builtin_fns() -> impl BuiltinFns<Self> {
        [
            (clear_pixel as BuiltinFn<Self>, "clear_pixel"),
            (delay_ms, "delay_ms"),
            (print, "print"),
            (set_pixel, "set_pixel"),
        ]
    }
}

pub struct PlatformContext<'r> {
    pub pixel_ops: Sender<'r>,
}

impl<'r> PlatformContext<'r> {
    pub fn new(pixel_ops: &'r crossbeam_channel::Sender<PixelOp>) -> Self {
        Self {
            pixel_ops: Sender { inner: pixel_ops },
        }
    }
}

pub struct Sender<'r> {
    pub inner: &'r crossbeam_channel::Sender<PixelOp>,
}

impl Sender<'_> {
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
    core_context: CoreContext,
    platform_context: &mut PlatformContext,
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (y, _) =
                core_context.data_stack.pop_specific::<value::Number>()?;
            let (x, _) =
                core_context.data_stack.pop_specific::<value::Number>()?;

            platform_context.pixel_ops.send(PixelOp::Clear([x.0, y.0]));

            Ok(BuiltinFnState::Completed)
        }
        _ => unreachable!(),
    }
}

fn delay_ms(
    step: usize,
    core_context: CoreContext,
    _: &mut PlatformContext,
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (delay_ms, _) =
                core_context.data_stack.pop_specific::<value::Number>()?;

            thread::sleep(Duration::from_millis(
                delay_ms.0.try_into().unwrap(),
            ));

            Ok(BuiltinFnState::Completed)
        }
        _ => unreachable!(),
    }
}

fn print(
    step: usize,
    core_context: CoreContext,
    _: &mut PlatformContext,
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let value = core_context.data_stack.pop_any()?;
            println!("{}", value.payload);
            core_context.data_stack.push(value);

            Ok(BuiltinFnState::Completed)
        }
        _ => unreachable!(),
    }
}

fn set_pixel(
    step: usize,
    core_context: CoreContext,
    platform_context: &mut PlatformContext,
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (y, _) =
                core_context.data_stack.pop_specific::<value::Number>()?;
            let (x, _) =
                core_context.data_stack.pop_specific::<value::Number>()?;

            platform_context.pixel_ops.send(PixelOp::Set([x.0, y.0]));

            Ok(BuiltinFnState::Completed)
        }
        _ => unreachable!(),
    }
}
