// I don't see the point of this lint. That weird case they present in its
// documentation certainly doesn't apply to me, and I've also never seen it
// anywhere else.
//
// We have a `clippy.toml` that is supposed to allow this for private modules,
// but it doesn't seem to work. Or I'm holding it wrong. I don't know.
#![allow(clippy::module_inception)]

pub mod intrinsics;
pub mod pipeline;
pub mod platform;
pub mod repr;
pub mod runtime;

pub use self::{
    repr::eval::value,
    runtime::{
        data_stack::DataStackResult,
        evaluator::RuntimeState,
        interpreter::Interpreter,
        namespaces::{PlatformFunction, PlatformFunctionState, RuntimeContext},
    },
};

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use capi_desktop::{
        core::Interpreter,
        loader::Loader,
        platform::{DesktopPlatform, PlatformContext},
    };

    #[test]
    fn native_capi_test_suite() -> anyhow::Result<()> {
        let mut interpreter = Interpreter::<DesktopPlatform>::new()?;

        let script_path = PathBuf::from("../tests.capi");
        let parent = None;

        let mut loader = Loader::new(&script_path)?;

        loader.load(&script_path, parent)?;
        let (_, _, code) = loader.updates().recv()??;
        let scripts = loader.wait_for_updated_scripts()?;

        interpreter.update(&code, parent, scripts)?;
        interpreter.run_tests(&mut PlatformContext::new(loader))?;

        Ok(())
    }
}
