use std::collections::VecDeque;

use super::a_tokenizer::Token;

pub fn parse(tokens: &mut VecDeque<Token>) -> Option<SyntaxElement> {
    match tokens.pop_front()? {
        Token::CurlyBracketOpen => {
            // not supported yet
            None
        }
        Token::Ident(ident) => Some(SyntaxElement::Word(ident)),
    }
}

pub enum SyntaxElement {
    Word(String),
}
