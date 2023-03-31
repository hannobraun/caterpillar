use std::vec;

use super::{
    analyze,
    tokenizer::{ExpectedToken, NoMoreTokens, Token, Tokens},
    Functions, ROOT_FN,
};

#[derive(Clone, Debug)]
pub struct SyntaxTree(Vec<SyntaxElement>);

impl IntoIterator for SyntaxTree {
    type Item = SyntaxElement;
    type IntoIter = vec::IntoIter<SyntaxElement>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    Function {
        name: String,
        body: SyntaxTree,
    },

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

pub fn parse(
    mut tokens: Tokens,
    functions: &mut Functions,
) -> Result<SyntaxTree, Error> {
    let mut syntax_tree = Vec::new();

    while let Ok(expression) = parse_expression(&mut tokens, functions) {
        syntax_tree.push(expression);
    }

    let syntax_tree = SyntaxTree(syntax_tree);
    let expression_graph = analyze(syntax_tree.clone(), functions);

    functions.insert(ROOT_FN.into(), expression_graph);

    Ok(syntax_tree)
}

fn parse_expression(
    tokens: &mut Tokens,
    functions: &mut Functions,
) -> Result<SyntaxElement, Error> {
    let expression = match tokens.peek()? {
        Token::Function => {
            let (name, body) = parse_function(tokens, functions)?;
            SyntaxElement::Function { name, body }
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

    Ok(expression)
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

        syntax_tree.push(expression);
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

        syntax_tree.push(expression)
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
