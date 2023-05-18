mod data_stack;
mod pipeline;

use std::collections::VecDeque;

pub use self::{
    data_stack::{DataStack, DataStackError},
    pipeline::{
        a_tokenizer::tokenize,
        b_parser::parse,
        d_evaluator::{evaluate, EvaluatorError},
    },
};

pub fn execute(
    code: &str,
    data_stack: &mut DataStack,
) -> Result<(), EvaluatorError> {
    let mut chars = code.chars().collect::<VecDeque<_>>();

    loop {
        let Some(token) = tokenize(&mut chars) else { break };
        let Some(syntax_element) = parse(token) else { continue };
        evaluate(syntax_element, data_stack)?;
    }

    Ok(())
}
