use std::iter;

use super::tokenizer::Token;

pub enum SyntaxTree {
    /// A function
    Fn { name: String },
}

pub fn parse(
    mut tokens: impl Iterator<Item = Token>,
) -> impl Iterator<Item = SyntaxTree> {
    iter::from_fn(move || {
        let Token::Fn { name } = tokens.next()?;
        Some(SyntaxTree::Fn { name })
    })
}
