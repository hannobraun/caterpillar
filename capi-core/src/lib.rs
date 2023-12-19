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
    use std::{
        fs::{self, File},
        io::Read,
        path::{Path, PathBuf},
    };

    use capi_desktop::{loader, platform::Context, Interpreter};

    #[test]
    fn native_capi_test_suite() -> anyhow::Result<()> {
        let script_path = PathBuf::from("../tests.capi");
        let code = loader::load(&script_path)?;

        let mut interpreter = Interpreter::new(&code)?;
        capi_desktop::platform::register(&mut interpreter);

        interpreter.run_tests(&mut Context::new(script_path))?;

        Ok(())
    }

    #[test]
    fn for_language_features() -> anyhow::Result<()> {
        run_tests_from_dir("../tests")
    }

    #[test]
    fn for_libraries() -> anyhow::Result<()> {
        run_tests_from_dir("../lib")
    }

    fn run_tests_from_dir(path: impl AsRef<Path>) -> anyhow::Result<()> {
        for dir_entry in fs::read_dir(path)? {
            let dir_entry = dir_entry?;

            println!("Running test suite `{}`...", dir_entry.path().display());

            let mut code = String::new();
            File::open(dir_entry.path())?.read_to_string(&mut code)?;

            let mut interpreter = Interpreter::new(&code)?;
            interpreter.run_tests(&mut Context::new(dir_entry.path()))?;
        }

        Ok(())
    }
}
