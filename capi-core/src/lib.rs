pub mod intrinsics;
pub mod pipeline;
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
        loader::Loader, platform::PlatformContext, Interpreter,
    };

    #[test]
    fn native_capi_test_suite() -> anyhow::Result<()> {
        let mut interpreter = Interpreter::new()?;
        capi_desktop::platform::register(&mut interpreter);

        let script_path = PathBuf::from("../tests.capi");
        let parent = None;

        let mut loader = Loader::new();
        let (code, _) = loader.load(&script_path)?;

        interpreter.update(&code, parent)?;
        interpreter
            .run_tests(&mut PlatformContext::new(script_path, loader))?;

        Ok(())
    }
}
