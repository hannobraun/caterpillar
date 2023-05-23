use crate::cp::syntax::{SyntaxElement, SyntaxTree};

use super::{a_tokenizer::Token, stage_input::StageInputReader, PipelineError};

pub fn parse(
    mut tokens: StageInputReader<Token>,
) -> Result<SyntaxElement, PipelineError<ParserError>> {
    let syntax_element = parse_syntax_element(&mut tokens)?;
    tokens.take();
    Ok(syntax_element)
}

fn parse_syntax_element(
    tokens: &mut StageInputReader<Token>,
) -> Result<SyntaxElement, PipelineError<ParserError>> {
    match tokens.peek()? {
        Token::CurlyBracketOpen => parse_block(tokens),
        Token::Ident(_) => {
            let word = parse_ident(tokens)?;
            Ok(SyntaxElement::Word(word))
        }
        _ => {
            let token = tokens.next()?;
            Err(PipelineError::Stage(ParserError::UnexpectedToken(
                token.clone(),
            )))
        }
    }
}

fn parse_block(
    tokens: &mut StageInputReader<Token>,
) -> Result<SyntaxElement, PipelineError<ParserError>> {
    expect_token(tokens, Token::CurlyBracketOpen)?;

    let mut syntax_tree = SyntaxTree {
        elements: Vec::new(),
    };

    loop {
        match tokens.peek()? {
            Token::CurlyBracketClose => {
                let _ = tokens.next();
                return Ok(SyntaxElement::Block { syntax_tree });
            }
            _ => {
                let syntax_element = parse_syntax_element(tokens)?;
                syntax_tree.elements.push(syntax_element)
            }
        }
    }
}

fn parse_ident(
    tokens: &mut StageInputReader<Token>,
) -> Result<String, PipelineError<ParserError>> {
    let token = tokens.next()?;
    let Token::Ident(ident) = token else {
        return Err(PipelineError::Stage(ParserError::UnexpectedToken(
            token.clone()
        )));
    };
    Ok(ident.clone())
}

fn expect_token(
    tokens: &mut StageInputReader<Token>,
    expected: Token,
) -> Result<(), PipelineError<ParserError>> {
    let token = tokens.next()?;

    if token != &expected {
        return Err(PipelineError::Stage(ParserError::UnexpectedToken(
            token.clone(),
        )));
    };

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Unexpected token: `{0:?}`")]
    UnexpectedToken(Token),
}
