use std::{iter, vec};

use crate::repr::{
    eval::value::ValuePayload,
    syntax::{SyntaxElement, SyntaxTree},
    tokens::{
        token::{self},
        Token,
    },
};

pub fn parse(tokens: Vec<Token>) -> ParserResult<SyntaxTree<SyntaxElement>> {
    let mut tokens = tokens.into_iter().peekable();
    parse_syntax_tree(None, &mut tokens)
}

fn parse_syntax_tree(
    terminator: Option<Token>,
    tokens: &mut Tokens,
) -> ParserResult<SyntaxTree<SyntaxElement>> {
    let mut syntax_tree = SyntaxTree::new();

    loop {
        let Some(syntax_element) =
            parse_syntax_element(terminator.as_ref(), tokens)?
        else {
            break;
        };

        syntax_tree.elements.push(syntax_element.clone());
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
        Token::Binding => {
            let names = parse_binding(tokens)?;
            SyntaxElement::Binding { names }
        }
        Token::SquareBracketOpen => {
            let syntax_tree = parse_array_expression(tokens)?;
            SyntaxElement::ArrayExpression(syntax_tree)
        }
        Token::CurlyBracketOpen => {
            let syntax_tree = parse_block_expression(tokens)?;
            SyntaxElement::BlockExpression(syntax_tree)
        }
        Token::Literal(_) => {
            let value = parse_literal(tokens)?;
            SyntaxElement::Literal(value)
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

fn parse_array_expression(
    tokens: &mut Tokens,
) -> ParserResult<SyntaxTree<SyntaxElement>> {
    expect::<token::SquareBracketOpen>(tokens)?;
    parse_syntax_tree(Some(Token::SquareBracketClose), tokens)
}

fn parse_binding(tokens: &mut Tokens) -> ParserResult<Vec<String>> {
    expect::<token::Binding>(tokens)?;
    expect::<token::SquareBracketOpen>(tokens)?;

    let mut symbols = Vec::new();

    for token in tokens {
        if token == Token::SquareBracketClose {
            break;
        }

        if let Token::Literal(ValuePayload::Symbol(symbol)) = token {
            symbols.push(symbol);
            continue;
        }

        return Err(ParserError::UnexpectedToken { actual: token });
    }

    Ok(symbols)
}

fn parse_block_expression(
    tokens: &mut Tokens,
) -> ParserResult<SyntaxTree<SyntaxElement>> {
    expect::<token::CurlyBracketOpen>(tokens)?;
    parse_syntax_tree(Some(Token::CurlyBracketClose), tokens)
}

fn parse_literal(tokens: &mut Tokens) -> ParserResult<ValuePayload> {
    let token = expect::<token::Literal>(tokens)?;
    Ok(token.0)
}

fn parse_word(tokens: &mut Tokens) -> ParserResult<String> {
    let token = expect::<token::Word>(tokens)?;
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
