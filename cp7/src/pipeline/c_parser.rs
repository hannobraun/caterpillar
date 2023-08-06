use crate::{
    pipeline::b_tokenizer::{token, NoMoreTokens, Token, Tokens},
    syntax::{Syntax, SyntaxElement, SyntaxFragment, SyntaxHandle, SyntaxTree},
    value::{self, Value},
};

pub fn parse(mut tokens: Tokens) -> ParserResult<(Syntax, SyntaxTree)> {
    let mut syntax = Syntax::new();
    let syntax_tree = parse_syntax_tree(None, &mut tokens, &mut syntax)?;

    Ok((syntax, syntax_tree))
}

fn parse_syntax_tree(
    terminator: Option<Token>,
    tokens: &mut Tokens,
    syntax: &mut Syntax,
) -> ParserResult<SyntaxTree> {
    let mut syntax_tree = SyntaxTree::new();

    while tokens.peek().is_ok() {
        match parse_fragment(terminator.clone(), tokens, syntax)? {
            Some(handle) => {
                let fragment = syntax.get(handle);
                syntax_tree.elements.push(fragment);
            }
            None => {
                break;
            }
        }
    }

    Ok(syntax_tree)
}

fn parse_fragment(
    terminator: Option<Token>,
    tokens: &mut Tokens,
    syntax: &mut Syntax,
) -> ParserResult<Option<SyntaxHandle>> {
    let syntax_element = match tokens.peek()? {
        Token::CurlyBracketOpen => {
            let block = parse_block(tokens, syntax)?;
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
        token => {
            if Some(&token) == terminator.as_ref() {
                tokens.next()?; // only peeked before; still need to consume
                return Ok(None);
            }

            return Err(ParserError::UnexpectedToken { actual: token });
        }
    };

    let handle = syntax.add(SyntaxFragment {
        payload: syntax_element,
        // This is a placeholder. At some point, this needs to point to the
        // syntax fragment that comes after this one, if this isn't the last in
        // the function.
        next: None,
    });

    Ok(Some(handle))
}

fn parse_block(
    tokens: &mut Tokens,
    syntax: &mut Syntax,
) -> ParserResult<SyntaxTree> {
    expect::<token::CurlyBracketOpen>(tokens)?;
    parse_syntax_tree(Some(Token::CurlyBracketClose), tokens, syntax)
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
