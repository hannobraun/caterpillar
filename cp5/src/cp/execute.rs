use std::{collections::VecDeque, ops::ControlFlow};

use super::{
    pipeline::{
        a_tokenizer::{tokenize, Token},
        b_parser::{parse, ParserError},
        d_evaluator::{evaluate, EvaluatorError},
        stage_input::StageInput,
        PipelineError,
    },
    DataStack,
};

pub fn execute(
    code: &str,
    data_stack: &mut DataStack,
    debug: bool,
) -> Result<(), Error> {
    let mut chars = code.chars().collect();
    let mut tokens = StageInput::new();

    loop {
        match execute_inner(&mut chars, &mut tokens, data_stack, debug) {
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
    tokens: &mut StageInput<Token>,
    data_stack: &mut DataStack,
    debug: bool,
) -> Result<ControlFlow<(), ()>, ErrorKind> {
    let Some(token) = tokenize(chars) else {
        return Ok(ControlFlow::Break(()))
    };
    if debug {
        dbg!(&token);
    }
    tokens.add(token);

    let syntax_element = match parse(tokens) {
        Ok(syntax_element) => syntax_element,
        Err(PipelineError::NotEnoughInput(_)) => {
            return Ok(ControlFlow::Continue(()))
        }
        Err(PipelineError::Stage(err)) => return Err(err.into()),
    };
    if debug {
        dbg!(&syntax_element);
    }

    evaluate(syntax_element, data_stack)?;

    Ok(ControlFlow::Continue(()))
}

#[derive(Debug, thiserror::Error)]
#[error("{kind}\n\t{chars:?}\n\t{tokens:?}")]
pub struct Error {
    pub kind: ErrorKind,
    pub chars: VecDeque<char>,
    pub tokens: StageInput<Token>,
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Parser error: {0}")]
    Parser(#[from] ParserError),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] EvaluatorError),
}
