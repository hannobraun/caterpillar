mod data_stack;
mod pipeline;

use std::collections::VecDeque;

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
        let mut word = String::new();

        while let Some(ch) = chars.pop_front() {
            if ch.is_whitespace() {
                break;
            }

            word.push(ch)
        }

        if word.is_empty() {
            break;
        }

        pipeline::d_evaluator::evaluate(&word, data_stack)?;
    }

    Ok(())
}
