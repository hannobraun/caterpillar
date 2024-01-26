use std::{path::PathBuf, thread, time::Duration};

use capi_core::{
    platform::Platform, repr::eval::fragments::FragmentId, value,
    DataStackResult, PlatformFunction, PlatformFunctionState, RuntimeContext,
};

use crate::loader::Loader;

pub struct DesktopPlatform;

impl Platform for DesktopPlatform {
    type Context = PlatformContext;
}

pub struct PlatformContext {
    /// The path of the script that was the entry point into the current program
    ///
    /// The entry script is the top-level script that was loaded first. Its path
    /// is used as the base for all `mod` declarations.
    pub entry_script_path: PathBuf,

    pub loader: Loader,
    pub pixel_ops: Sender,
    pub loading_script: Option<Option<FragmentId>>,
}

impl PlatformContext {
    pub fn new(entry_script_path: impl Into<PathBuf>, loader: Loader) -> Self {
        let (pixel_ops, _) = crossbeam_channel::unbounded();

        Self {
            entry_script_path: entry_script_path.into(),
            loader,
            pixel_ops: Sender { inner: pixel_ops },
            loading_script: None,
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

pub fn register(
) -> impl IntoIterator<Item = (PlatformFunction<PlatformContext>, &'static str)>
{
    [
        (
            clear_pixel as PlatformFunction<PlatformContext>,
            "clear_pixel",
        ),
        (delay_ms, "delay_ms"),
        (mod_, "mod"),
        (print, "print"),
        (set_pixel, "set_pixel"),
    ]
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

fn mod_(
    runtime_context: RuntimeContext,
    platform_context: &mut PlatformContext,
) -> DataStackResult<PlatformFunctionState> {
    let (path_segments, _) =
        runtime_context.data_stack.pop_specific::<value::Array>()?;

    let mut path = platform_context.entry_script_path.clone();
    path.pop(); // remove script itself, so we're left with the folder it's in

    let num_segments = path_segments.0.len();

    for (i, segment) in path_segments.0.into_iter().enumerate() {
        let segment = segment.expect::<value::Symbol>()?;

        let is_last_segment = i == num_segments - 1;
        let segment = if is_last_segment {
            format!("{}.capi", segment.0)
        } else {
            segment.0
        };

        path.push(segment);
    }

    // The error handling here is not great, but we can only return
    // `DataStackError`. It might be best to make the return value platform-
    // specific too. Then we can return a platform-specific error value.
    let parent = Some(runtime_context.word);
    platform_context.loader.load(path, parent).unwrap();
    platform_context.loading_script = Some(parent);

    Ok(PlatformFunctionState::Sleeping)
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
