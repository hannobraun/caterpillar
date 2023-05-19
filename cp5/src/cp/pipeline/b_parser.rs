use std::collections::VecDeque;

use super::a_tokenizer::Token;

pub fn parse(
    tokens: &mut VecDeque<Token>,
) -> Option<Result<SyntaxElement, ParserError>> {
    match tokens.pop_front()? {
        Token::Ident(ident) => Some(Ok(SyntaxElement::Word(ident))),
        token => Some(Err(ParserError::UnexpectedToken(token))),
    }
}

pub enum SyntaxElement {
    Word(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Unexpected token: `{0:?}`")]
    UnexpectedToken(Token),
}
