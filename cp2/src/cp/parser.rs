use std::{collections::BTreeMap, slice};

use super::{
    tokenizer::{ExpectedToken, NoMoreTokens, Token, Tokens},
    ROOT_FN,
};

#[derive(Clone, Debug)]
pub struct SyntaxTree(Vec<SyntaxElement>);

impl<'r> IntoIterator for &'r SyntaxTree {
    type Item = &'r SyntaxElement;
    type IntoIter = slice::Iter<'r, SyntaxElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    /// Binds values from the stack to provided names
    Binding(Vec<String>),

    Array {
        syntax_tree: SyntaxTree,
    },

    /// A block of code that is lazily evaluated
    Block {
        syntax_tree: SyntaxTree,
    },

    /// A word refers to a function or variable
    Word(String),
}

pub type Functions = BTreeMap<String, SyntaxTree>;

pub fn parse(
    mut tokens: Tokens,
    functions: &mut Functions,
) -> Result<SyntaxTree, Error> {
    let mut syntax_tree = Vec::new();

    while let Ok(expression) = parse_expression(&mut tokens, functions) {
        syntax_tree.extend(expression);
    }

    let syntax_tree = SyntaxTree(syntax_tree);

    functions.insert(ROOT_FN.into(), syntax_tree.clone());

    Ok(syntax_tree)
}

fn parse_expression(
    tokens: &mut Tokens,
    functions: &mut Functions,
) -> Result<Option<SyntaxElement>, Error> {
    let expression = match tokens.peek()? {
        Token::Function => {
            let (name, body) = parse_function(tokens, functions)?;
            functions.insert(name, body);
            return Ok(None);
        }
        Token::BindingOperator => {
            let binding_names = parse_binding(tokens)?;
            SyntaxElement::Binding(binding_names)
        }
        Token::CurlyBracketOpen => {
            let syntax_tree = parse_block(tokens, functions)?;
            SyntaxElement::Block { syntax_tree }
        }
        Token::SquareBracketOpen => {
            let syntax_tree = parse_array(tokens, functions)?;
            SyntaxElement::Array { syntax_tree }
        }
        Token::Ident(_) => {
            let ident = tokens.expect_ident()?;
            SyntaxElement::Word(ident)
        }
        _ => {
            let token = tokens.next()?;
            return Err(Error::UnexpectedToken(token));
        }
    };

    Ok(Some(expression))
}

fn parse_function(
    tokens: &mut Tokens,
    functions: &mut Functions,
) -> Result<(String, SyntaxTree), Error> {
    tokens.expect(Token::Function)?;
    let name = tokens.expect_ident()?;
    let body = parse_block(tokens, functions)?;
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

fn parse_block(
    tokens: &mut Tokens,
    functions: &mut Functions,
) -> Result<SyntaxTree, Error> {
    let mut syntax_tree = Vec::new();

    tokens.expect(Token::CurlyBracketOpen)?;

    loop {
        let expression = match tokens.peek()? {
            Token::CurlyBracketClose => {
                tokens.next()?;
                break;
            }
            _ => parse_expression(tokens, functions)?,
        };

        syntax_tree.extend(expression);
    }

    Ok(SyntaxTree(syntax_tree))
}

fn parse_array(
    tokens: &mut Tokens,
    functions: &mut Functions,
) -> Result<SyntaxTree, Error> {
    let mut syntax_tree = Vec::new();

    tokens.expect(Token::SquareBracketOpen)?;

    loop {
        let expression = match tokens.peek()? {
            Token::SquareBracketClose => {
                tokens.next()?;
                break;
            }
            _ => parse_expression(tokens, functions)?,
        };

        syntax_tree.extend(expression)
    }

    Ok(SyntaxTree(syntax_tree))
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
