use crate::syntax::{Syntax, SyntaxHandle};

use super::stages::{a_loader::load, b_tokenizer::tokenize, c_parser::parse};

pub fn run(
    path: impl AsRef<std::path::Path>,
    syntax: &mut Syntax,
) -> anyhow::Result<Option<SyntaxHandle>> {
    let code = load(path)?;
    let tokens = tokenize(&code);
    let start = parse(tokens, syntax)?;

    Ok(start)
}
