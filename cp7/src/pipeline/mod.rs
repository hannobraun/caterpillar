mod run;
mod stages;

pub use self::run::run;

pub fn start(path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
    let mut syntax = crate::syntax::Syntax::new();

    let start = run(path, &mut syntax)?;

    if let Some(start) = start {
        let mut evaluator = crate::runtime::evaluator::Evaluator::new(start);
        while evaluator.step(&syntax)? {}
    }

    Ok(())
}
