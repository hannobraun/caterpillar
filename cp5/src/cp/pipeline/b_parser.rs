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
    let syntax_element = match tokens.peek()? {
        Token::CurlyBracketOpen => {
            let syntax_tree = parse_block(tokens)?;
            SyntaxElement::Block { syntax_tree }
        }
        Token::Fn => {
            let (name, body) = parse_fn(tokens)?;
            SyntaxElement::Function { name, body }
        }
        Token::Ident(_) => {
            let ident = parse_ident(tokens)?;
            SyntaxElement::Word(ident)
        }
        token => {
            return Err(PipelineError::Stage(ParserError::UnexpectedToken(
                token.clone(),
            )));
        }
    };

    Ok(syntax_element)
}

fn parse_block(
    tokens: &mut StageInputReader<Token>,
) -> Result<SyntaxTree, PipelineError<ParserError>> {
    expect_token(tokens, Token::CurlyBracketOpen)?;

    let mut syntax_tree = SyntaxTree {
        elements: Vec::new(),
    };

    loop {
        match tokens.peek()? {
            Token::CurlyBracketClose => {
                let _ = tokens.next();
                return Ok(syntax_tree);
            }
            _ => {
                let syntax_element = parse_syntax_element(tokens)?;
                syntax_tree.elements.push(syntax_element)
            }
        }
    }
}

fn parse_fn(
    tokens: &mut StageInputReader<Token>,
) -> Result<(String, SyntaxTree), PipelineError<ParserError>> {
    expect_token(tokens, Token::Fn)?;
    let name = parse_ident(tokens)?;
    let body = parse_block(tokens)?;

    Ok((name, body))
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
