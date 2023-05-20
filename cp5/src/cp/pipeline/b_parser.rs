use std::collections::VecDeque;

use crate::cp::syntax::{SyntaxElement, SyntaxTree};

use super::{a_tokenizer::Token, PipelineError};

pub fn parse(
    tokens: &mut VecDeque<Token>,
) -> Result<SyntaxElement, PipelineError<ParserError>> {
    match tokens.pop_front().ok_or(PipelineError::NotEnoughInput)? {
        token @ Token::CurlyBracketOpen => {
            tokens.push_front(token);
            parse_block(tokens)
        }
        Token::Ident(ident) => Ok(SyntaxElement::Word(ident)),
        token => Err(PipelineError::Stage(ParserError::UnexpectedToken(token))),
    }
}

fn parse_block(
    tokens: &mut VecDeque<Token>,
) -> Result<SyntaxElement, PipelineError<ParserError>> {
    let open = tokens.pop_front().ok_or(PipelineError::NotEnoughInput)?;
    let Token::CurlyBracketOpen = open else {
        return Err(PipelineError::Stage(ParserError::UnexpectedToken(open)));
    };

    let mut syntax_tree = SyntaxTree {
        elements: Vec::new(),
    };

    loop {
        match tokens.pop_front().ok_or(PipelineError::NotEnoughInput)? {
            Token::CurlyBracketClose => {
                return Ok(SyntaxElement::Block { syntax_tree });
            }
            token => {
                tokens.push_front(token);
                let syntax_element = parse(tokens)?;
                syntax_tree.elements.push(syntax_element)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Unexpected token: `{0:?}`")]
    UnexpectedToken(Token),
}
