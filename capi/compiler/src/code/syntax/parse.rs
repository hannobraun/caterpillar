use std::result;

use crate::code::{
    tokens::{Keyword::*, NoMoreTokens, Punctuator::*, Token, Tokens},
    Index, IndexMap, Signature,
};

use super::{
    repr::types::SyntaxType, Binding, Branch, BranchLocation, Expression, Function, FunctionLocation, Member, MemberLocation, NamedFunction, Parameter
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

    let location = FunctionLocation::Named { index };
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
    parameters: &mut IndexMap<Parameter>,
) -> Result<()> {
    while let Some(parameter) = parse_branch_parameter(tokens)? {
        parameters.push(parameter);

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

fn parse_branch_parameter(tokens: &mut Tokens) -> Result<Option<Parameter>> {
    let parameter = match tokens.take()? {
        Token::Identifier { name } => {
            Some(Parameter::Binding(Binding { name }))
        }
        Token::IntegerLiteral { value } => Some(Parameter::Literal {
            value: value.into(),
        }),
        Token::Punctuator(Transformer) => None,
        token => {
            return Err(Error::UnexpectedToken { actual: token });
        }
    };

    Ok(parameter)
}

fn parse_branch_body(
    tokens: &mut Tokens,
    body: &mut IndexMap<Member>,
    location: BranchLocation,
) -> Result<()> {
    loop {
        match tokens.peek()? {
            Token::Punctuator(BranchStart) | Token::Keyword(End) => {
                break;
            }
            _ => {
                let location = MemberLocation {
                    parent: Box::new(location.clone()),
                    index: body.next_index(),
                };
                let member = parse_member(tokens, location)?;
                body.push(member);
            }
        }
    }

    Ok(())
}

fn parse_member(
    tokens: &mut Tokens,
    location: MemberLocation,
) -> Result<Member> {
    if let Token::Comment { text } = tokens.peek()? {
        let text = text.clone();
        tokens.take()?;
        return Ok(Member::Comment { text });
    }

    let (expression, signature) = parse_expression(tokens, location)?;

    let syntax_node = Member::Expression {
        expression,
        signature,
    };

    Ok(syntax_node)
}

fn parse_expression(
    tokens: &mut Tokens,
    location: MemberLocation,
) -> Result<(Expression, Option<Signature<SyntaxType>>)> {
    let expression = if let Token::Keyword(Fn) = tokens.peek()? {
        let location = FunctionLocation::Local { location };
        parse_function(tokens, location)
            .map(|function| Expression::LocalFunction { function })?
    } else {
        match tokens.take()? {
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

    Ok((expression, signature))
}

fn parse_type_annotation(
    tokens: &mut Tokens,
) -> Result<Option<Signature<SyntaxType>>> {
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
) -> Result<Signature<SyntaxType>> {
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
        // We already handle the terminator at the end of the loop. We have to
        // also handle it here, since it's allowed to not have any outputs,
        // which would make the terminator the first token we encounter in this
        // loop.
        if *tokens.peek()? == terminator {
            tokens.take()?;
            break;
        }

        let type_ = parse_type(tokens)?;
        outputs.push(type_);

        match tokens.take()? {
            Token::Punctuator(Delimiter) => {
                continue;
            }
            token if token == terminator => {
                break;
            }
            token => {
                return Err(Error::UnexpectedToken { actual: token });
            }
        }
    }

    Ok(Signature { inputs, outputs })
}

fn parse_type(tokens: &mut Tokens) -> Result<SyntaxType> {
    let type_ = match tokens.take()? {
        Token::Identifier { name } => SyntaxType::Identifier { name },
        Token::Keyword(Fn) => {
            let terminator = Token::Keyword(End);
            let signature = parse_signature(tokens, terminator)?;
            SyntaxType::Function { signature }
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
