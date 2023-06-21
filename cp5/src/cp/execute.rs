use std::convert::Infallible;

use super::{
    pipeline::{
        a_tokenizer::tokenize,
        b_parser::{parse, ParserError},
        c_analyzer::{analyze, Expression},
        d_evaluator::{evaluate, EvaluatorError},
        stage_input::{NoMoreInput, StageInput},
        PipelineError,
    },
    syntax::SyntaxElement,
    tokens::Token,
    Bindings, DataStack, Functions,
};

pub fn execute(
    code: &str,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), Error> {
    let mut chars = code.chars().collect();
    let mut tokens = StageInput::new();
    let mut syntax_elements = StageInput::new();
    let mut expressions = StageInput::new();

    loop {
        match execute_inner(
            &mut chars,
            &mut tokens,
            &mut syntax_elements,
            &mut expressions,
            data_stack,
            bindings,
            functions,
            tests,
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

#[allow(clippy::too_many_arguments)]
fn execute_inner(
    chars: &mut StageInput<char>,
    tokens: &mut StageInput<Token>,
    syntax_elements: &mut StageInput<SyntaxElement>,
    expressions: &mut StageInput<Expression>,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), ErrorKind> {
    let token = tokenize(chars.reader())?;
    tokens.add(token);

    let syntax_element = parse(tokens.reader())?;
    syntax_elements.add(syntax_element);

    let expression = analyze(syntax_elements.reader(), functions)?;
    expressions.add(expression);

    evaluate(expressions.reader(), data_stack, bindings, functions, tests)?;

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
