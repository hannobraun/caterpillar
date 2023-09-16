use std::{iter, vec};

use crate::language::repr::{
    syntax::{SyntaxElement, SyntaxTree},
    tokens::{token, Token},
};

pub fn parse(tokens: Vec<Token>) -> ParserResult<SyntaxTree> {
    let mut tokens = tokens.into_iter().peekable();
    parse_syntax_tree(None, &mut tokens)
}

fn parse_syntax_tree(
    terminator: Option<Token>,
    tokens: &mut Tokens,
) -> ParserResult<SyntaxTree> {
    let mut syntax_tree = SyntaxTree::new();

    loop {
        let Some(syntax_element) =
            parse_syntax_element(terminator.as_ref(), tokens)?
        else {
            break;
        };

        syntax_tree.elements.push(syntax_element);
    }

    Ok(syntax_tree)
}

fn parse_syntax_element(
    terminator: Option<&Token>,
    tokens: &mut Tokens,
) -> ParserResult<Option<SyntaxElement>> {
    let Some(next_token) = tokens.peek() else {
        return Ok(None);
    };

    let syntax_element = match next_token {
        Token::CurlyBracketOpen => {
            let syntax_tree = parse_block(tokens)?;
            SyntaxElement::Block(syntax_tree)
        }
        Token::Number(_) => {
            let number = parse_number(tokens)?;
            SyntaxElement::Number(number)
        }
        Token::Symbol(_) => {
            let symbol = parse_symbol(tokens)?;
            SyntaxElement::Symbol(symbol)
        }
        Token::Word(_) => {
            let word = parse_word(tokens)?;
            SyntaxElement::Word(word)
        }
        _ => {
            // Only peeked before; still need to consume.
            let token = tokens.next().unwrap();

            if Some(&token) == terminator {
                return Ok(None);
            }

            return Err(ParserError::UnexpectedToken { actual: token });
        }
    };

    Ok(Some(syntax_element))
}

fn parse_block(tokens: &mut Tokens) -> ParserResult<SyntaxTree> {
    expect::<token::CurlyBracketOpen>(tokens)?;
    parse_syntax_tree(Some(Token::CurlyBracketClose), tokens)
}

fn parse_word(tokens: &mut Tokens) -> ParserResult<String> {
    let token = expect::<token::Word>(tokens)?;
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

pub fn expect<T>(tokens: &mut Tokens) -> ParserResult<T>
where
    T: TryFrom<Token, Error = Token>,
{
    let token = tokens.next().ok_or(NoMoreTokens)?;

    let token = token
        .try_into()
        .map_err(|token| ParserError::UnexpectedToken { actual: token })?;

    Ok(token)
}

pub type Tokens<'r> = iter::Peekable<vec::IntoIter<Token>>;

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Expected more tokens")]
    ExpectedMoreTokens(#[from] NoMoreTokens),

    #[error("Unexpected token: {actual:?}")]
    UnexpectedToken { actual: Token },
}

#[derive(Debug, thiserror::Error)]
#[error("No more tokens")]
pub struct NoMoreTokens;
