#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use capi_core::runtime::interpreter::Interpreter;
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
        interpreter.run_tests(PlatformContext::new(&pixel_ops))?;

        Ok(())
    }
}
