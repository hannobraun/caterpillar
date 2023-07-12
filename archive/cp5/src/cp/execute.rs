use std::convert::Infallible;

use super::{
    pipeline::{
        channel::{NoMoreInput, PipelineChannel},
        ir::{
            analyzer_output::AnalyzerEvent, syntax::SyntaxElement,
            tokens::Token,
        },
        stages::{
            a_tokenizer::tokenize,
            b_parser::{parse, ParserError},
            c_analyzer::{analyze, AnalyzerError},
            d_evaluator::evaluate,
        },
    },
    Bindings, DataStack, EvaluatorError, Functions, PipelineError,
};

pub fn execute(
    code: &str,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), Error> {
    let mut chars = code.chars().collect();
    let mut tokens = PipelineChannel::new();
    let mut syntax_elements = PipelineChannel::new();
    let mut expressions = PipelineChannel::new();

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
    chars: &mut PipelineChannel<char>,
    tokens: &mut PipelineChannel<Token>,
    syntax_elements: &mut PipelineChannel<SyntaxElement>,
    analyzer_events: &mut PipelineChannel<AnalyzerEvent>,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    functions: &mut Functions,
    tests: &mut Functions,
) -> Result<(), ErrorKind> {
    let token = tokenize(chars.as_input())?;
    tokens.as_output().push(token);

    let syntax_element = parse(tokens.as_input())?;
    syntax_elements.as_output().push(syntax_element);

    let analyzer_event =
        analyze(syntax_elements.as_input(), bindings, functions, tests)?;
    analyzer_events.as_output().push(analyzer_event);

    evaluate(
        analyzer_events.as_input(),
        data_stack,
        bindings,
        functions,
        tests,
    )?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
#[error("{kind}\n\t{syntax_elements:?}\n\t{tokens:?}\n\t{chars:?}")]
pub struct Error {
    pub kind: ErrorKind,
    pub syntax_elements: PipelineChannel<SyntaxElement>,
    pub tokens: PipelineChannel<Token>,
    pub chars: PipelineChannel<char>,
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Not enough input")]
    NotEnoughInput,

    #[error("Parser error: {0}")]
    Parser(#[from] ParserError),

    #[error("Analyzer error: {0}")]
    Analyzer(#[from] AnalyzerError),

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

impl From<PipelineError<AnalyzerError>> for ErrorKind {
    fn from(err: PipelineError<AnalyzerError>) -> Self {
        match err {
            PipelineError::NotEnoughInput(NoMoreInput) => Self::NotEnoughInput,
            PipelineError::Stage(err) => Self::Analyzer(err),
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
