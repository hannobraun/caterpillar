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

    let mut names = Vec::new();

    // I don't love how this is using square brackets to delimit the names of a
    // binding. We need *something* here (although a single delimiter at the end
    // would also work; not necessary to actually bracket it), but having the
    // square brackets, implies that this is matching against an actual array on
    // the stack, which of course it isn't.
    //
    // Earlier prototypes used a `.` at the end, which looks nice, but requires
    // an additional type of token. Also, I'm considering using `.` to terminate
    // expressions. Both uses would conflict, as you might want to employ a
    // binding as part of an expression, not at the end of one.
    //
    // Thoughts:
    //
    // - *Not* terminating with the binding seems needlessly obtuse in the way
    //   it complicates the expression. Maybe it's just fine to forbid that on a
    //   syntax level.
    // - Maybe the answer is to not have a delimiter for either of these, and
    //   use significant whitespace instead?
    //
    // More notes on that idea of using significant whitespace:
    //
    // - With bindings, the requirement would be to have the names be on the
    //   same line or indented. To end the binding, you write something into the
    //   next line, at the same or a lower indentation level.
    // - Expressions could end in much the same way? It's even conceivable to
    //   have multiple nested expressions (defined by levels of indentation),
    //   with restrictions in place on which expressions can access the results
    //   of which other expressions.
    // - Higher expressions must be able to access the results of lower
    //   expressions, but the other way around could be limited?
    // - What I'm hoping for, is a way to make it super clear what's happening
    //   with the data stack, without requiring bindings and terminated
    //   expressions everywhere. But I have no idea if that would actually work.
    for token in tokens {
        if token == Token::SquareBracketClose {
            break;
        }

        if let Token::Literal(ValuePayload::Symbol(name)) = token {
            names.push(name);
            continue;
        }

        return Err(ParserError::UnexpectedToken { actual: token });
    }

    Ok(names)
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
