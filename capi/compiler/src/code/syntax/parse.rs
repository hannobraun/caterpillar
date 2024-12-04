use std::result;

use crate::code::{
    tokens::{Keyword::*, NoMoreTokens, Punctuator::*, Token, Tokens},
    BranchLocation, ExpressionLocation, FunctionLocation, Index, IndexMap,
};

use super::{
    repr::types::Type, AnnotatedExpression, Branch, Expression, Function,
    NamedFunction, Pattern, Signature,
};

/// # Parse the provided tokens
///
/// ## Implementation Note
///
/// This compiler pass currently panics when it encounters an unexpected token.
/// It would be better, if it encoded the error into its output instead. This is
/// non-trivial though, compared to other compiler passes that do that.
///
/// For example, if an identifier can not be resolved, this only affects that
/// identifier, and it is quite easy to encode that by having a [`Expression`]
/// variant for unresolved identifiers.
///
/// In the case of parsing, however, an unexpected token would likely result in
/// the parser not knowing what to do with the following tokens, even it can be
/// taught to recover eventually. Those tokens would also have to be encoded
/// into the code representation. Otherwise, viewing that representation (which
/// would include the error) makes no sense to a user.
///
/// I'm inclined to leave this be for now. Panicking certainly works well enough
/// in the current setup. Once we have a code database, it will no longer do.
/// But then we also need a completely different parser architecture.
///
/// It's probably not worth solving this non-trivial problem for the current
/// architecture, for little gain, only to re-solve it again for the new
/// architecture, once that is necessary.
pub fn parse(mut tokens: Tokens) -> IndexMap<NamedFunction> {
    let mut named_functions = IndexMap::default();

    loop {
        let index = named_functions.next_index();

        let function = match parse_named_function(&mut tokens, index) {
            Ok(function) => function,
            Err(Error::NoMoreTokens(NoMoreTokens)) => {
                break;
            }
            Err(err) => {
                panic!("Parser error: {err:?}");
            }
        };

        let actual_index = named_functions.push(function);
        assert_eq!(
            index, actual_index,
            "Function has a different index than was initially assumed.",
        );
    }

    named_functions
}

fn parse_named_function(
    tokens: &mut Tokens,
    index: Index<NamedFunction>,
) -> Result<NamedFunction> {
    let name = parse_function_name(tokens)?;

    let location = FunctionLocation::NamedFunction { index };
    let function = parse_function(tokens, location)?;

    Ok(NamedFunction {
        name,
        inner: function,
    })
}

fn parse_function_name(tokens: &mut Tokens) -> Result<String> {
    let name = loop {
        if let Token::Comment { .. } = tokens.peek()? {
            // Comments in the top-level context are currently ignored.
            tokens.take()?;
            continue;
        }

        match tokens.take()? {
            Token::Identifier { name } => {
                break name;
            }
            token => {
                return Err(Error::UnexpectedToken { actual: token });
            }
        }
    };

    match tokens.take()? {
        Token::Punctuator(Introducer) => {}
        token => {
            return Err(Error::UnexpectedToken { actual: token });
        }
    }

    Ok(name)
}

fn parse_function(
    tokens: &mut Tokens,
    location: FunctionLocation,
) -> Result<Function> {
    let mut function = Function::default();

    match tokens.take()? {
        Token::Keyword(Fn) => {}
        token => {
            return Err(Error::UnexpectedToken { actual: token });
        }
    }

    loop {
        let location = BranchLocation {
            parent: Box::new(location.clone()),
            index: function.branches.next_index(),
        };

        let Some(branch) = parse_branch(tokens, location)? else {
            break;
        };

        function.branches.push(branch);
    }

    match tokens.take()? {
        Token::Keyword(End) => {}
        token => {
            return Err(Error::UnexpectedToken { actual: token });
        }
    }

    Ok(function)
}

fn parse_branch(
    tokens: &mut Tokens,
    location: BranchLocation,
) -> Result<Option<Branch>> {
    match tokens.peek()? {
        Token::Punctuator(BranchStart) => {
            tokens.take()?;
        }
        Token::Keyword(End) => {
            return Ok(None);
        }
        _ => {
            let token = tokens.take()?;
            return Err(Error::UnexpectedToken { actual: token });
        }
    }

    let mut branch = Branch::default();

    parse_branch_parameters(tokens, &mut branch.parameters)?;
    parse_branch_body(tokens, &mut branch.body, location)?;

    Ok(Some(branch))
}

fn parse_branch_parameters(
    tokens: &mut Tokens,
    parameters: &mut Vec<Pattern>,
) -> Result<()> {
    while let Some(pattern) = parse_branch_parameter(tokens)? {
        parameters.push(pattern);

        match tokens.take()? {
            Token::Punctuator(Delimiter) => {
                // If we have a delimiter, then we're good here. Next loop
                // iteration, we'll either parse the next parameter, or if it
                // was the last one, find the start of the branch body.
                continue;
            }
            Token::Punctuator(Transformer) => {
                // The last parameter doesn't need a delimiter, so this is fine
                // too.
                break;
            }
            token => {
                return Err(Error::UnexpectedToken { actual: token });
            }
        }
    }

    Ok(())
}

fn parse_branch_parameter(tokens: &mut Tokens) -> Result<Option<Pattern>> {
    let pattern = match tokens.take()? {
        Token::Identifier { name } => Some(Pattern::Identifier { name }),
        Token::IntegerLiteral { value } => Some(Pattern::Literal {
            value: value.into(),
        }),
        Token::Punctuator(Transformer) => None,
        token => {
            return Err(Error::UnexpectedToken { actual: token });
        }
    };

    Ok(pattern)
}

fn parse_branch_body(
    tokens: &mut Tokens,
    body: &mut IndexMap<AnnotatedExpression>,
    location: BranchLocation,
) -> Result<()> {
    loop {
        match tokens.peek()? {
            Token::Punctuator(BranchStart) | Token::Keyword(End) => {
                break;
            }
            _ => {
                let location = ExpressionLocation {
                    parent: Box::new(location.clone()),
                    index: body.next_index(),
                };
                let expression = parse_expression(tokens, location)?;
                body.push(expression);
            }
        }
    }

    Ok(())
}

fn parse_expression(
    tokens: &mut Tokens,
    location: ExpressionLocation,
) -> Result<AnnotatedExpression> {
    let expression = if let Token::Keyword(Fn) = tokens.peek()? {
        let location = FunctionLocation::AnonymousFunction { location };
        parse_function(tokens, location)
            .map(|function| Expression::LocalFunction { function })?
    } else {
        match tokens.take()? {
            Token::Comment { text } => Expression::Comment { text },
            Token::Identifier { name } => Expression::Identifier { name },
            Token::IntegerLiteral { value } => Expression::LiteralNumber {
                value: value.into(),
            },
            token => {
                return Err(Error::UnexpectedToken { actual: token });
            }
        }
    };

    let signature = parse_type_annotation(tokens)?;

    let expression = AnnotatedExpression {
        inner: expression,
        signature,
    };

    Ok(expression)
}

fn parse_type_annotation(tokens: &mut Tokens) -> Result<Option<Signature>> {
    let Token::Punctuator(Introducer) = tokens.peek()? else {
        return Ok(None);
    };
    tokens.take()?;

    let terminator = Token::Punctuator(Terminator);
    let signature = parse_signature(tokens, terminator)?;

    Ok(Some(signature))
}

fn parse_signature(
    tokens: &mut Tokens,
    terminator: Token,
) -> Result<Signature> {
    let mut inputs = Vec::new();

    loop {
        if let Token::Punctuator(Transformer) = tokens.peek()? {
            tokens.take()?;
            break;
        }

        let type_ = parse_type(tokens)?;
        inputs.push(type_);

        match tokens.take()? {
            Token::Punctuator(Delimiter) => {
                continue;
            }
            Token::Punctuator(Transformer) => {
                break;
            }
            token => {
                return Err(Error::UnexpectedToken { actual: token });
            }
        }
    }

    let mut outputs = Vec::new();

    loop {
        if *tokens.peek()? == terminator {
            tokens.take()?;
            break;
        }

        let type_ = parse_type(tokens)?;
        outputs.push(type_);
    }

    Ok(Signature { inputs, outputs })
}

fn parse_type(tokens: &mut Tokens) -> Result<Type> {
    let type_ = match tokens.take()? {
        Token::Identifier { name } => Type::Identifier { name },
        Token::Keyword(Fn) => {
            let terminator = Token::Keyword(End);
            let signature = parse_signature(tokens, terminator)?;
            Type::Function { signature }
        }
        token => {
            return Err(Error::UnexpectedToken { actual: token });
        }
    };

    Ok(type_)
}

type Result<T> = result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    NoMoreTokens(#[from] NoMoreTokens),

    #[error("Unexpected token: {actual:?}")]
    UnexpectedToken { actual: Token },
}
