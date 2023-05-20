mod data_stack;
mod pipeline;
mod syntax;

use std::{collections::VecDeque, ops::ControlFlow};

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
) -> Result<(), ErrorKind> {
    let mut chars = code.chars().collect::<VecDeque<_>>();
    let mut tokens = VecDeque::new();

    loop {
        match execute_inner(&mut chars, &mut tokens, data_stack) {
            Ok(ControlFlow::Continue(())) => continue,
            Ok(ControlFlow::Break(())) => break,
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

fn execute_inner(
    chars: &mut VecDeque<char>,
    tokens: &mut VecDeque<pipeline::a_tokenizer::Token>,
    data_stack: &mut DataStack,
) -> Result<ControlFlow<(), ()>, ErrorKind> {
    let Some(token) = tokenize(chars) else {
        return Ok(ControlFlow::Break(()))
    };
    tokens.push_back(token);

    let Some(syntax_element) = parse(tokens) else {
        return Ok(ControlFlow::Continue(()))
    };
    let syntax_element = syntax_element?;

    evaluate(syntax_element, data_stack)?;

    Ok(ControlFlow::Continue(()))
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Parser error: {0}")]
    Parser(#[from] pipeline::b_parser::ParserError),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] EvaluatorError),
}
