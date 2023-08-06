use crate::tokenizer::{token, NoMoreTokens, Token, Tokens};

pub fn parse(mut tokens: Tokens) -> ParserResult<SyntaxTree> {
    let mut syntax_elements = Vec::new();

    while let Ok(token) = tokens.peek() {
        let syntax_element = parse_syntax_element(token.clone(), &mut tokens)?;
        syntax_elements.push(syntax_element);
    }

    Ok(SyntaxTree {
        elements: syntax_elements,
    })
}

fn parse_syntax_element(
    next_token: Token,
    tokens: &mut Tokens,
) -> ParserResult<SyntaxElement> {
    match next_token {
        Token::CurlyBracketOpen => {
            let block = parse_block(tokens)?;
            Ok(SyntaxElement::Block(block))
        }
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

fn parse_block(tokens: &mut Tokens) -> ParserResult<SyntaxTree> {
    expect::<token::CurlyBracketOpen>(tokens)?;

    let mut syntax_elements = Vec::new();

    loop {
        match tokens.peek()? {
            Token::CurlyBracketClose => {
                tokens.next()?; // only peeked before; still need to consume
                break;
            }
            token => {
                let syntax_element = parse_syntax_element(token, tokens)?;
                syntax_elements.push(syntax_element);
            }
        }
    }

    Ok(SyntaxTree {
        elements: syntax_elements,
    })
}

fn parse_fn_ref(tokens: &mut Tokens) -> ParserResult<String> {
    let token = expect::<token::FnRef>(tokens)?;
    Ok(token.0)
}

fn parse_symbol(tokens: &mut Tokens) -> ParserResult<String> {
    let token = expect::<token::Symbol>(tokens)?;
    Ok(token.0)
}

fn expect<T>(tokens: &mut Tokens) -> ParserResult<T>
where
    T: TryFrom<Token, Error = Token>,
{
    match tokens.next()?.try_into() {
        Ok(token) => Ok(token),
        Err(token) => Err(ParserError::UnexpectedToken { actual: token }),
    }
}

#[derive(Debug)]
pub struct SyntaxTree {
    pub elements: Vec<SyntaxElement>,
}

#[derive(Debug)]
pub enum SyntaxElement {
    Block(SyntaxTree),
    FnRef(String),
    Symbol(String),
}

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Expected more tokens")]
    ExpectedMoreTokens(#[from] NoMoreTokens),

    #[error("Unexpected token: {actual:?}")]
    UnexpectedToken { actual: Token },
}
