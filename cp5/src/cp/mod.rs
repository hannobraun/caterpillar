mod data_stack;
mod pipeline;

use std::collections::VecDeque;

use self::pipeline::a_tokenizer::tokenize;
pub use self::{
    data_stack::{DataStack, DataStackError},
    pipeline::d_evaluator::EvaluatorError,
};

pub fn execute(
    code: &str,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    let mut chars = code.chars().collect::<VecDeque<_>>();

    loop {
        let Some(token) = tokenize(&mut chars) else { break };
        pipeline::d_evaluator::evaluate(token, data_stack)?;
    }

    Ok(())
}
