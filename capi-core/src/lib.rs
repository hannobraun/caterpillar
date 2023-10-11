pub mod intrinsics;
pub mod pipeline;
pub mod repr;
pub mod runtime;

pub use self::{
    repr::eval::value,
    runtime::{
        data_stack::DataStackResult,
        evaluator::RuntimeState,
        namespaces::{FunctionState, PlatformFunction, RuntimeContext},
        interpreter::Interpreter,
    },
};

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Read,
    };

    use crate::Interpreter;

    #[test]
    fn capi_tests() -> anyhow::Result<()> {
        for dir_entry in fs::read_dir("../tests")? {
            let dir_entry = dir_entry?;

            println!("Running test suite `{}`...", dir_entry.path().display());

            let mut code = String::new();
            File::open(dir_entry.path())?.read_to_string(&mut code)?;

            let mut interpreter = Interpreter::new(&code)?;
            interpreter.run_tests()?;
        }

        Ok(())
    }
}
