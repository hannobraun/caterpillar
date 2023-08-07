use std::path::Path;

use crate::{pipeline, syntax::Syntax};

use super::evaluator::Evaluator;

pub fn start(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let mut syntax = Syntax::new();

    let start = pipeline::run(path, &mut syntax)?;

    if let Some(start) = start {
        let mut evaluator = Evaluator::new(start);
        while evaluator.step(&syntax)? {}
    }

    Ok(())
}
