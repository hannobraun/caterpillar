use crate::tokenizer::{Token, Tokens};

pub fn parse(mut tokens: Tokens) -> ParserResult<Vec<SyntaxElement>> {
    let mut syntax_elements = Vec::new();

    while let Some(token) = tokens.inner.front() {
        let syntax_element = parse_syntax_element(token.clone(), &mut tokens)?;
        syntax_elements.push(syntax_element);
    }

    Ok(syntax_elements)
}

fn parse_syntax_element(
    next_token: Token,
    tokens: &mut Tokens,
) -> ParserResult<SyntaxElement> {
    match next_token {
        Token::CurlyBracketOpen => panic!("Parsing block not supported"),
        Token::FnRef(_) => {
            let fn_ref = parse_fn_ref(tokens)?;
            Ok(SyntaxElement::FnRef(fn_ref))
        }
        Token::Symbol(_) => {
            let symbol = parse_symbol(tokens)?;
            Ok(SyntaxElement::Symbol(symbol))
        }
        token => Err(ParserError::UnexpectedToken { actual: token }),
    }
}

fn parse_fn_ref(tokens: &mut Tokens) -> ParserResult<String> {
    match tokens.inner.pop_front().unwrap() {
        Token::FnRef(fn_ref) => Ok(fn_ref),
        token => Err(ParserError::UnexpectedToken { actual: token }),
    }
}

fn parse_symbol(tokens: &mut Tokens) -> ParserResult<String> {
    match tokens.inner.pop_front().unwrap() {
        Token::Symbol(symbol) => Ok(symbol),
        token => Err(ParserError::UnexpectedToken { actual: token }),
    }
}

pub enum SyntaxElement {
    FnRef(String),
    Symbol(String),
}

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Unexpected token: {actual:?}")]
    UnexpectedToken { actual: Token },
}
