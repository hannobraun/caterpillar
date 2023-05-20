use std::collections::VecDeque;

use crate::cp::syntax::{SyntaxElement, SyntaxTree};

use super::{a_tokenizer::Token, PipelineError};

pub fn parse(
    tokens: &mut VecDeque<Token>,
) -> Result<SyntaxElement, PipelineError<ParserError>> {
    match tokens.front().ok_or(PipelineError::NotEnoughInput)? {
        Token::CurlyBracketOpen => parse_block(tokens),
        Token::Ident(_) => {
            let word = parse_word(tokens)?;
            Ok(SyntaxElement::Word(word))
        }
        _ => {
            let token =
                tokens.pop_front().ok_or(PipelineError::NotEnoughInput)?;
            Err(PipelineError::Stage(ParserError::UnexpectedToken(token)))
        }
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

fn parse_word(
    tokens: &mut VecDeque<Token>,
) -> Result<String, PipelineError<ParserError>> {
    let token = tokens.pop_front().ok_or(PipelineError::NotEnoughInput)?;
    let Token::Ident(ident) = token else {
        return Err(PipelineError::Stage(ParserError::UnexpectedToken(token)));
    };
    Ok(ident)
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Unexpected token: `{0:?}`")]
    UnexpectedToken(Token),
}
