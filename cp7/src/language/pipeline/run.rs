use std::io;

use crate::{
    language::syntax::{Syntax, SyntaxHandle},
    loader::load::load,
};

use super::stages::{
    a_tokenizer::tokenize,
    b_parser::{parse, ParserError},
};

pub fn run(
    path: impl AsRef<std::path::Path>,
    syntax: &mut Syntax,
) -> Result<Option<SyntaxHandle>, PipelineError> {
    let code = load(path)?;
    let tokens = tokenize(&code);
    let start = parse(tokens, syntax)?;

    Ok(start)
}

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Failed to load code from file")]
    Loader(#[from] io::Error),

    #[error("Failed to parse")]
    Parser(#[from] ParserError),
}
