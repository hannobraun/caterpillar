use std::collections::VecDeque;

use super::a_tokenizer::Token;

pub fn parse(tokens: &mut VecDeque<Token>) -> Option<String> {
    let Token::Ident(ident) = tokens.pop_front()?;
    Some(ident)
}
