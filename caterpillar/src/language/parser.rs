use std::iter;

use super::tokenizer::Token;

pub enum SyntaxTree {
    /// A function
    Fn { name: String },
}

pub fn parse(
    tokens: &mut dyn Iterator<Item = Token>,
) -> impl Iterator<Item = SyntaxTree> + '_ {
    parse_tokens(tokens)
}

fn parse_tokens(
    mut tokens: &mut dyn Iterator<Item = Token>,
) -> impl Iterator<Item = SyntaxTree> + '_ {
    iter::from_fn(move || {
        for token in &mut tokens {
            match token {
                Token::Fn { name } => {
                    return Some(SyntaxTree::Fn { name });
                }
                Token::ArrayOpen => {}
                Token::ArrayClose => {}
            }
        }

        None
    })
}
