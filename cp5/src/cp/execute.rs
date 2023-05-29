use std::convert::Infallible;

use super::{
    pipeline::{
        a_tokenizer::{tokenize, Token},
        b_parser::{parse, ParserError},
        d_evaluator::{evaluate, EvaluatorError},
        stage_input::{NoMoreInput, StageInput},
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
            Ok(()) | Err(ErrorKind::NotEnoughInput) => {
                if chars.is_empty() {
                    break;
                }

                continue;
            }
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
    chars: &mut StageInput<char>,
    tokens: &mut StageInput<Token>,
    syntax_elements: &mut StageInput<SyntaxElement>,
    data_stack: &mut DataStack,
    functions: &mut Functions,
    tests: &mut Functions,
    debug: bool,
) -> Result<(), ErrorKind> {
    let token = tokenize(chars.reader())?;
    if debug {
        dbg!(&token);
    }
    tokens.add(token);

    let syntax_element = parse(tokens.reader())?;
    if debug {
        dbg!(&syntax_element);
    }
    syntax_elements.add(syntax_element);

    evaluate(syntax_elements.reader(), data_stack, functions, tests)?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
#[error("{kind}\n\t{syntax_elements:?}\n\t{tokens:?}\n\t{chars:?}")]
pub struct Error {
    pub kind: ErrorKind,
    pub syntax_elements: StageInput<SyntaxElement>,
    pub tokens: StageInput<Token>,
    pub chars: StageInput<char>,
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Not enough input")]
    NotEnoughInput,

    #[error("Parser error: {0}")]
    Parser(#[from] ParserError),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] EvaluatorError),
}

impl From<PipelineError<Infallible>> for ErrorKind {
    fn from(err: PipelineError<Infallible>) -> Self {
        match err {
            PipelineError::NotEnoughInput(NoMoreInput) => Self::NotEnoughInput,
            PipelineError::Stage(_) => {
                unreachable!("Handling infallible error")
            }
        }
    }
}

impl From<PipelineError<ParserError>> for ErrorKind {
    fn from(err: PipelineError<ParserError>) -> Self {
        match err {
            PipelineError::NotEnoughInput(NoMoreInput) => Self::NotEnoughInput,
            PipelineError::Stage(err) => Self::Parser(err),
        }
    }
}

impl From<PipelineError<EvaluatorError>> for ErrorKind {
    fn from(err: PipelineError<EvaluatorError>) -> Self {
        match err {
            PipelineError::NotEnoughInput(NoMoreInput) => Self::NotEnoughInput,
            PipelineError::Stage(err) => Self::Evaluator(err),
        }
    }
}
