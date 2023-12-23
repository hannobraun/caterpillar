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
        namespaces::{FunctionState, PlatformFunction, RuntimeContext},
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
        let script_path = PathBuf::from("../tests.capi");
        let (code, _) = Loader::new().load(&script_path)?;

        let mut interpreter = Interpreter::new()?;
        capi_desktop::platform::register(&mut interpreter);

        interpreter.update(&code)?;
        interpreter.run_tests(&mut PlatformContext::new(script_path))?;

        Ok(())
    }
}
