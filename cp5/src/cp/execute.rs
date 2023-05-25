use std::{collections::VecDeque, ops::ControlFlow};

use super::{
    pipeline::{
        a_tokenizer::{tokenize, Token},
        b_parser::{parse, ParserError},
        d_evaluator::{evaluate, EvaluatorError},
        stage_input::StageInput,
        PipelineError,
    },
    syntax::SyntaxElement,
    DataStack, Functions,
};

pub fn execute(
    code: &str,
    data_stack: &mut DataStack,
    functions: &mut Functions,
    tests: &mut Functions,
    debug: bool,
) -> Result<(), Error> {
    let mut chars = code.chars().collect();
    let mut tokens = StageInput::new();
    let mut syntax_elements = StageInput::new();

    loop {
        match execute_inner(
            &mut chars,
            &mut tokens,
            &mut syntax_elements,
            data_stack,
            functions,
            tests,
            debug,
        ) {
            Ok(ControlFlow::Continue(())) => continue,
            Ok(ControlFlow::Break(())) => break,
            Err(kind) => {
                return Err(Error {
                    kind,
                    syntax_elements,
                    tokens,
                    chars,
                })
            }
        }
    }

    Ok(())
}

fn execute_inner(
    chars: &mut VecDeque<char>,
    tokens: &mut StageInput<Token>,
    syntax_elements: &mut StageInput<SyntaxElement>,
    data_stack: &mut DataStack,
    functions: &mut Functions,
    tests: &mut Functions,
    debug: bool,
) -> Result<ControlFlow<(), ()>, ErrorKind> {
    let Some(token) = tokenize(chars) else {
        return Ok(ControlFlow::Break(()))
    };
    if debug {
        dbg!(&token);
    }
    tokens.add(token);

    let syntax_element = match parse(tokens.reader()) {
        Ok(syntax_element) => syntax_element,
        Err(PipelineError::NotEnoughInput(_)) => {
            return Ok(ControlFlow::Continue(()))
        }
        Err(PipelineError::Stage(err)) => return Err(err.into()),
    };
    if debug {
        dbg!(&syntax_element);
    }
    syntax_elements.add(syntax_element);

    match evaluate(syntax_elements.reader(), data_stack, functions, tests) {
        Ok(()) => {}
        Err(PipelineError::NotEnoughInput(_)) => {
            return Ok(ControlFlow::Continue(()))
        }
        Err(PipelineError::Stage(err)) => return Err(err.into()),
    }

    Ok(ControlFlow::Continue(()))
}

#[derive(Debug, thiserror::Error)]
#[error("{kind}\n\t{syntax_elements:?}\n\t{tokens:?}\n\t{chars:?}")]
pub struct Error {
    pub kind: ErrorKind,
    pub syntax_elements: StageInput<SyntaxElement>,
    pub tokens: StageInput<Token>,
    pub chars: VecDeque<char>,
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Parser error: {0}")]
    Parser(#[from] ParserError),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] EvaluatorError),
}
