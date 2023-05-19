use std::collections::VecDeque;

use crate::cp::syntax::{SyntaxElement, SyntaxTree};

use super::a_tokenizer::Token;

pub fn parse(
    tokens: &mut VecDeque<Token>,
) -> Option<Result<SyntaxElement, ParserError>> {
    match tokens.pop_front()? {
        token @ Token::CurlyBracketOpen => {
            tokens.push_front(token);
            parse_block(tokens)
        }
        Token::Ident(ident) => Some(Ok(SyntaxElement::Word(ident))),
        token => Some(Err(ParserError::UnexpectedToken(token))),
    }
}

fn parse_block(
    tokens: &mut VecDeque<Token>,
) -> Option<Result<SyntaxElement, ParserError>> {
    let open = tokens.pop_front()?;
    let Token::CurlyBracketOpen = open else {
        return Some(Err(ParserError::UnexpectedToken(open)));
    };

    let mut syntax_tree = SyntaxTree {
        elements: Vec::new(),
    };

    loop {
        match tokens.pop_front()? {
            Token::CurlyBracketClose => {
                return Some(Ok(SyntaxElement::Block { syntax_tree }));
            }
            token => {
                tokens.push_front(token);
                match parse(tokens)? {
                    Ok(syntax_element) => {
                        syntax_tree.elements.push(syntax_element)
                    }
                    Err(err) => return Some(Err(err)),
                }
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Unexpected token: `{0:?}`")]
    UnexpectedToken(Token),
}
