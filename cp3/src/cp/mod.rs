mod call_stack;
mod data_stack;
mod expressions;
mod functions;
mod keywords;
mod pipeline;
mod syntax;
mod tokens;
mod values;

pub use self::{
    call_stack::CallStack,
    data_stack::{DataStack, Error as DataStackError},
    functions::Functions,
    pipeline::{evaluate, EvaluatorError},
};

pub fn execute(
    code: impl IntoIterator<Item = char>,
    functions: &mut Functions,
) -> Result<DataStack, Error> {
    let mut data_stack = DataStack::new();

    let tokens = tokens::Tokens({
        let mut tokens = Vec::new();

        let pipeline = pipeline::Pipeline {
            tokenizer: pipeline::a_tokenizer::tokenizer(),
        };

        let tokenizer =
            code.into_iter().fold(pipeline.tokenizer, |tokenizer, ch| {
                let (tokenizer, ts) =
                    pipeline::a_tokenizer::push_char(ch, tokenizer);
                tokens.extend(ts);
                tokenizer
            });

        let end_of_stream = '\u{0004}';
        let (_, ts) =
            pipeline::a_tokenizer::push_char(end_of_stream, tokenizer);
        tokens.extend(ts);

        tokens.into()
    });
    let syntax_tree = pipeline::parse(tokens)?;
    let expressions = pipeline::analyze("", syntax_tree, functions);

    pipeline::evaluate(
        expressions,
        functions,
        &mut CallStack,
        &mut data_stack,
    )?;

    Ok(data_stack)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
pub enum Error {
    #[error("Parser error: {0}")]
    Parser(#[from] pipeline::b_parser::Error),

    #[error("Evaluator error: {0}")]
    Evaluator(#[from] pipeline::d_evaluator::Error),
}
