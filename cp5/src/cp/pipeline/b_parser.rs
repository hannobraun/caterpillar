use crate::cp::syntax::{SyntaxElement, SyntaxTree};

use super::{a_tokenizer::Token, stage_input::StageInput, PipelineError};

pub fn parse(
    tokens: &mut StageInput<Token>,
) -> Result<SyntaxElement, PipelineError<ParserError>> {
    match tokens.reader().peek()? {
        Token::CurlyBracketOpen => parse_block(tokens),
        Token::Ident(_) => {
            let word = parse_word(tokens)?;
            Ok(SyntaxElement::Word(word))
        }
        _ => {
            let token = tokens.reader().next()?;
            Err(PipelineError::Stage(ParserError::UnexpectedToken(token)))
        }
    }
}

fn parse_block(
    tokens: &mut StageInput<Token>,
) -> Result<SyntaxElement, PipelineError<ParserError>> {
    let open = tokens.reader().next()?;
    let Token::CurlyBracketOpen = open else {
        return Err(PipelineError::Stage(ParserError::UnexpectedToken(open)));
    };

    let mut syntax_tree = SyntaxTree {
        elements: Vec::new(),
    };

    loop {
        match tokens.reader().peek()? {
            Token::CurlyBracketClose => {
                let _ = tokens.reader().next();
                return Ok(SyntaxElement::Block { syntax_tree });
            }
            _ => {
                let syntax_element = parse(tokens)?;
                syntax_tree.elements.push(syntax_element)
            }
        }
    }
}

fn parse_word(
    tokens: &mut StageInput<Token>,
) -> Result<String, PipelineError<ParserError>> {
    let token = tokens.reader().next()?;
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
