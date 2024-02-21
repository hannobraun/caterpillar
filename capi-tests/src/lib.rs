#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use capi_core::runtime::{
        interpreter::Interpreter, test_runner::run_tests,
    };
    use capi_desktop::{
        loader::Loader,
        platform::{DesktopPlatform, PlatformContext},
    };

    #[test]
    fn native_capi_test_suite() -> anyhow::Result<()> {
        let script_path = PathBuf::from("../tests.capi");
        let (_, scripts) = Loader::new(script_path)?;
        let mut interpreter = Interpreter::<DesktopPlatform>::new(scripts)?;
        let (pixel_ops, _) = crossbeam_channel::unbounded();

        interpreter.update()?;
        let report =
            run_tests(&mut interpreter, PlatformContext::new(&pixel_ops))?;

        let mut failed = false;
        for (name, pass) in report.inner {
            if !pass {
                println!("Test failure: {name}");
                failed = true;
            }
        }

        if failed {
            panic!();
        }

        Ok(())
    }
}
