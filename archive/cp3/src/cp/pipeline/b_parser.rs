use crate::cp::{
    keywords::Keyword,
    syntax::{SyntaxElement, SyntaxTree},
    tokens::{ExpectedToken, NoMoreTokens, Token, Tokens},
};

pub fn parse(mut tokens: Tokens) -> Result<SyntaxTree, Error> {
    let mut syntax_tree = Vec::new();

    loop {
        let Some(expression) = parse_expression(&mut tokens)? else {
            break;
        };
        syntax_tree.push(expression);
    }

    Ok(SyntaxTree::from(syntax_tree))
}

fn parse_expression(
    tokens: &mut Tokens,
) -> Result<Option<SyntaxElement>, Error> {
    let next_token = match tokens.peek() {
        Ok(token) => token,
        Err(NoMoreTokens) => return Ok(None),
    };

    let expression = match next_token {
        Token::BindingOperator => {
            let binding_names = parse_binding(tokens)?;
            SyntaxElement::Binding(binding_names)
        }
        Token::CurlyBracketOpen => {
            let syntax_tree = parse_block(tokens)?;
            SyntaxElement::Block { syntax_tree }
        }
        Token::SquareBracketOpen => {
            let syntax_tree = parse_array(tokens)?;
            SyntaxElement::Array { syntax_tree }
        }
        Token::Keyword(Keyword::Fn) => {
            let (name, body) = parse_function(tokens)?;
            SyntaxElement::Function { name, body }
        }
        Token::Keyword(Keyword::Test) => {
            let (name, body) = parse_test(tokens)?;
            SyntaxElement::Test { name, body }
        }
        Token::Keyword(Keyword::Mod) => {
            let (name, body) = parse_module(tokens)?;
            SyntaxElement::Module { name, body }
        }
        Token::Ident(_) => {
            let ident = tokens.expect_ident()?;
            SyntaxElement::Word(ident)
        }
        Token::String(_) => {
            let string = tokens.expect_string()?;
            SyntaxElement::String(string)
        }
        _ => {
            let token = tokens.next()?;
            return Err(Error::UnexpectedToken(token));
        }
    };

    Ok(Some(expression))
}

fn parse_function(tokens: &mut Tokens) -> Result<(String, SyntaxTree), Error> {
    tokens.expect(Token::Keyword(Keyword::Fn))?;
    let name = tokens.expect_ident()?;
    let body = parse_block(tokens)?;
    Ok((name, body))
}

fn parse_test(tokens: &mut Tokens) -> Result<(String, SyntaxTree), Error> {
    tokens.expect(Token::Keyword(Keyword::Test))?;
    let name = tokens.expect_string()?;
    let body = parse_block(tokens)?;
    Ok((name, body))
}

fn parse_module(tokens: &mut Tokens) -> Result<(String, SyntaxTree), Error> {
    tokens.expect(Token::Keyword(Keyword::Mod))?;
    let name = tokens.expect_ident()?;
    let body = parse_block(tokens)?;
    Ok((name, body))
}

fn parse_binding(tokens: &mut Tokens) -> Result<Vec<String>, Error> {
    let mut binding_names = Vec::new();

    tokens.expect(Token::BindingOperator)?;

    loop {
        match tokens.next()? {
            Token::Ident(ident) => binding_names.push(ident),
            Token::Period => break,
            token => return Err(Error::UnexpectedToken(token)),
        }
    }

    Ok(binding_names)
}

fn parse_block(tokens: &mut Tokens) -> Result<SyntaxTree, Error> {
    let mut syntax_tree = Vec::new();

    tokens.expect(Token::CurlyBracketOpen)?;

    loop {
        let expression = match tokens.peek()? {
            Token::CurlyBracketClose => {
                tokens.next()?;
                break;
            }
            _ => parse_expression(tokens)?,
        };
        let expression =
            expression.ok_or(Error::ExpectedMoreTokens(NoMoreTokens))?;

        syntax_tree.push(expression);
    }

    Ok(SyntaxTree::from(syntax_tree))
}

fn parse_array(tokens: &mut Tokens) -> Result<SyntaxTree, Error> {
    let mut syntax_tree = Vec::new();

    tokens.expect(Token::SquareBracketOpen)?;

    loop {
        let expression = match tokens.peek()? {
            Token::SquareBracketClose => {
                tokens.next()?;
                break;
            }
            _ => parse_expression(tokens)?,
        };
        let expression =
            expression.ok_or(Error::ExpectedMoreTokens(NoMoreTokens))?;

        syntax_tree.push(expression)
    }

    Ok(SyntaxTree::from(syntax_tree))
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
pub enum Error {
    #[error("Expected more tokens")]
    ExpectedMoreTokens(#[from] NoMoreTokens),

    #[error(transparent)]
    ExpectedToken(#[from] ExpectedToken),

    #[error("Unexpected token: `{0:?}`")]
    UnexpectedToken(Token),
}
