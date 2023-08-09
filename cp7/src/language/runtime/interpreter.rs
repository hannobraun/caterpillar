use crate::language::{
    pipeline::{self, PipelineError},
    syntax::Syntax,
};

use super::evaluator::Evaluator;

pub struct Interpreter {
    pub syntax: Syntax,
    pub evaluator: Evaluator,
}

impl Interpreter {
    pub fn new(code: &str) -> Result<Option<Self>, PipelineError> {
        let mut syntax = Syntax::new();
        let start = pipeline::run(code, &mut syntax)?;

        let Some(start) = start else { return Ok(None); };
        let evaluator = Evaluator::new(start);

        Ok(Some(Interpreter { syntax, evaluator }))
    }
}
