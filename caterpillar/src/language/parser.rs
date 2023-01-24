use std::iter;

use super::tokenizer::Token;

pub enum Syntax {
    /// A function
    Fn { name: String },
}

pub fn parse(
    mut tokens: impl Iterator<Item = Token>,
) -> impl Iterator<Item = Syntax> {
    iter::from_fn(move || {
        let Token::Fn { name: function } = tokens.next()?;
        Some(Syntax::Fn { name: function })
    })
}
