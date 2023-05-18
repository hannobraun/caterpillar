use std::collections::VecDeque;

use super::a_tokenizer::Token;

pub fn parse(tokens: &mut VecDeque<Token>) -> Option<String> {
    match tokens.pop_front()? {
        Token::Ident(ident) => Some(ident),
    }
}
