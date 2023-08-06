use crate::{
    pipeline::b_tokenizer::{token, NoMoreTokens, Token, Tokens},
    syntax::{Syntax, SyntaxElement, SyntaxFragment, SyntaxTree},
    value::{self, Value},
};

pub fn parse(mut tokens: Tokens) -> ParserResult<(Syntax, SyntaxTree)> {
    let mut syntax = Syntax::new();
    let mut syntax_tree = SyntaxTree::new();

    while let Ok(token) = tokens.peek() {
        let fragment = parse_fragment(token, &mut tokens)?;

        syntax_tree.elements.push(fragment.clone());
        syntax.add(fragment);
    }

    Ok((syntax, syntax_tree))
}

fn parse_fragment(
    next_token: Token,
    tokens: &mut Tokens,
) -> ParserResult<SyntaxFragment> {
    let syntax_element = match next_token {
        Token::CurlyBracketOpen => {
            let block = parse_block(tokens)?;
            SyntaxElement::Value(value::Block(block).into())
        }
        Token::FnRef(_) => {
            let fn_ref = parse_fn_ref(tokens)?;
            SyntaxElement::FnRef(fn_ref)
        }
        Token::Number(_) => {
            let number = parse_number(tokens)?;
            SyntaxElement::Value(Value::Number(number))
        }
        Token::Symbol(_) => {
            let symbol = parse_symbol(tokens)?;
            SyntaxElement::Value(value::Symbol(symbol).into())
        }
        token => return Err(ParserError::UnexpectedToken { actual: token }),
    };

    Ok(SyntaxFragment {
        payload: syntax_element,
    })
}

fn parse_block(tokens: &mut Tokens) -> ParserResult<SyntaxTree> {
    expect::<token::CurlyBracketOpen>(tokens)?;

    let mut syntax_tree = SyntaxTree::new();

    loop {
        match tokens.peek()? {
            Token::CurlyBracketClose => {
                tokens.next()?; // only peeked before; still need to consume
                break;
            }
            token => {
                let syntax_element = parse_fragment(token, tokens)?;
                syntax_tree.elements.push(syntax_element);
            }
        }
    }

    Ok(syntax_tree)
}

fn parse_fn_ref(tokens: &mut Tokens) -> ParserResult<String> {
    let token = expect::<token::FnRef>(tokens)?;
    Ok(token.0)
}

fn parse_number(tokens: &mut Tokens) -> ParserResult<i64> {
    let token = expect::<token::Number>(tokens)?;
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

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Expected more tokens")]
    ExpectedMoreTokens(#[from] NoMoreTokens),

    #[error("Unexpected token: {actual:?}")]
    UnexpectedToken { actual: Token },
}
