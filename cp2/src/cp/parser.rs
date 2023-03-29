use super::tokenizer::{ExpectedToken, NoMoreTokens, Token, Tokens};

#[derive(Clone, Debug)]
pub struct SyntaxTree(pub Vec<Expression>);

#[derive(Clone, Debug)]
pub enum Expression {
    Function {
        name: String,
        body: SyntaxTree,
    },

    /// Binds values from the stack to provided names
    Binding(Vec<String>),

    Array(SyntaxTree),

    /// A block of code that is lazily evaluated
    Block(SyntaxTree),

    /// A word refers to a function or variable
    Word(String),
}

pub fn parse(mut tokens: Tokens) -> Result<SyntaxTree, Error> {
    let mut expressions = Vec::new();

    while let Ok(expression) = parse_expression(&mut tokens) {
        expressions.push(expression);
    }

    Ok(SyntaxTree(expressions))
}

fn parse_expression(tokens: &mut Tokens) -> Result<Expression, Error> {
    match tokens.peek()? {
        Token::Function => {
            let (name, body) = parse_function(tokens)?;
            Ok(Expression::Function { name, body })
        }
        Token::BindingOperator => {
            let binding_names = parse_binding(tokens)?;
            Ok(Expression::Binding(binding_names))
        }
        Token::CurlyBracketOpen => {
            let expressions = parse_block(tokens)?;
            Ok(Expression::Block(expressions))
        }
        Token::SquareBracketOpen => {
            let syntax_tree = parse_array(tokens)?;
            Ok(Expression::Array(syntax_tree))
        }
        Token::Ident(_) => {
            let ident = tokens.expect_ident()?;
            Ok(Expression::Word(ident))
        }
        _ => {
            let token = tokens.next()?;
            Err(Error::UnexpectedToken(token))
        }
    }
}

fn parse_function(tokens: &mut Tokens) -> Result<(String, SyntaxTree), Error> {
    tokens.expect(Token::Function)?;
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
    let mut expressions = Vec::new();

    tokens.expect(Token::CurlyBracketOpen)?;

    loop {
        let expression = match tokens.peek()? {
            Token::CurlyBracketClose => {
                tokens.next()?;
                break;
            }
            _ => parse_expression(tokens)?,
        };

        expressions.push(expression);
    }

    Ok(SyntaxTree(expressions))
}

fn parse_array(tokens: &mut Tokens) -> Result<SyntaxTree, Error> {
    let mut expressions = Vec::new();

    tokens.expect(Token::SquareBracketOpen)?;

    loop {
        let expression = match tokens.peek()? {
            Token::SquareBracketClose => {
                tokens.next()?;
                break;
            }
            _ => parse_expression(tokens)?,
        };

        expressions.push(expression)
    }

    Ok(SyntaxTree(expressions))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Expected more tokens")]
    ExpectedMoreTokens(#[from] NoMoreTokens),

    #[error(transparent)]
    ExpectedToken(#[from] ExpectedToken),

    #[error("Unexpected token: `{0:?}`")]
    UnexpectedToken(Token),
}
