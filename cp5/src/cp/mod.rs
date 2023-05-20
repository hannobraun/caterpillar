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

pub fn execute(code: &str, data_stack: &mut DataStack) -> Result<(), Error> {
    let mut chars = code.chars().collect::<VecDeque<_>>();
    let mut tokens = VecDeque::new();

    loop {
        match execute_inner(&mut chars, &mut tokens, data_stack) {
            Ok(ControlFlow::Continue(())) => continue,
            Ok(ControlFlow::Break(())) => break,
            Err(kind) => {
                return Err(Error {
                    kind,
                    chars,
                    tokens,
                })
            }
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
#[error("{kind}\n\t{chars:?}\n\t{tokens:?}")]
pub struct Error {
    pub kind: ErrorKind,
    pub chars: VecDeque<char>,
    pub tokens: VecDeque<pipeline::a_tokenizer::Token>,
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Parser error: {0}")]
    Parser(#[from] pipeline::b_parser::ParserError),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] EvaluatorError),
}
