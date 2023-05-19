mod data_stack;
mod pipeline;
mod syntax;

use std::collections::VecDeque;

pub use self::{
    data_stack::{DataStack, DataStackError},
    pipeline::{
        a_tokenizer::tokenize,
        b_parser::parse,
        d_evaluator::{evaluate, EvaluatorError},
    },
};

pub fn execute(code: &str, data_stack: &mut DataStack) -> Result<(), Error> {
    let mut chars = code.chars().collect::<VecDeque<_>>();
    let mut tokens = VecDeque::new();

    loop {
        let Some(token) = tokenize(&mut chars) else { break };
        tokens.push_back(token);

        let Some(syntax_element) = parse(&mut tokens) else { continue };
        let syntax_element = syntax_element?;

        evaluate(syntax_element, data_stack)?;
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Parser error: {0}")]
    Parser(#[from] pipeline::b_parser::ParserError),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] EvaluatorError),
}
