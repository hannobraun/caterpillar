use std::result;

use crate::code::{
    tokens::{Keyword::*, Punctuator::*, Token, Tokens},
    Branch, BranchLocation, ConcreteSignature, Expression, ExpressionLocation,
    Function, FunctionLocation, Index, IndexMap, NamedFunction, Pattern, Type,
    TypedExpression,
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

        let Ok(function) = parse_named_function(&mut tokens, index) else {
            break;
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
        if let Some(Token::Comment { .. }) = tokens.peek() {
            // Comments in the top-level context are currently ignored.
            tokens.take();
            continue;
        }

        match tokens.take().ok_or(())? {
            Token::Identifier { name } => {
                break name;
            }
            token => {
                panic!("Unexpected token: {token:?}");
            }
        }
    };

    match tokens.take().ok_or(())? {
        Token::Punctuator(Introducer) => {}
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    Ok(name)
}

fn parse_function(
    tokens: &mut Tokens,
    location: FunctionLocation,
) -> Result<Function> {
    let mut function = Function::default();

    match tokens.take().ok_or(())? {
        Token::Keyword(Fn) => {}
        token => {
            panic!("Unexpected token: {token:?}");
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

    match tokens.take().ok_or(())? {
        Token::Keyword(End) => {}
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    Ok(function)
}

fn parse_branch(
    tokens: &mut Tokens,
    location: BranchLocation,
) -> Result<Option<Branch>> {
    match tokens.peek().ok_or(())? {
        Token::Punctuator(BranchStart) => {
            tokens.take();
        }
        Token::Keyword(End) => {
            return Ok(None);
        }
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    let mut branch = Branch::default();

    parse_branch_parameters(tokens, &mut branch);
    parse_branch_body(tokens, &mut branch, location)?;

    Ok(Some(branch))
}

fn parse_branch_parameters(tokens: &mut Tokens, branch: &mut Branch) {
    loop {
        let Some(token) = tokens.take() else {
            break;
        };

        match parse_branch_parameter(token) {
            Ok(pattern) => {
                branch.parameters.push(pattern);
            }
            Err(()) => {
                break;
            }
        }

        let Some(token) = tokens.take() else {
            break;
        };

        match token {
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
                panic!("Unexpected token: {token:?}");
            }
        }
    }
}

fn parse_branch_parameter(token: Token) -> Result<Pattern> {
    match token {
        Token::Identifier { name } => Ok(Pattern::Identifier { name }),
        Token::IntegerLiteral { value } => Ok(Pattern::Literal {
            value: value.into(),
        }),
        Token::Punctuator(Transformer) => Err(()),
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }
}

fn parse_branch_body(
    tokens: &mut Tokens,
    branch: &mut Branch,
    location: BranchLocation,
) -> Result<()> {
    while let Some(token) = tokens.peek() {
        match token {
            Token::Punctuator(BranchStart) | Token::Keyword(End) => {
                break;
            }
            _ => {
                let location = ExpressionLocation {
                    parent: Box::new(location.clone()),
                    index: branch.body.next_index(),
                };
                let expression = parse_expression(tokens, location)?;
                branch.body.push(expression);
            }
        }
    }

    Ok(())
}

fn parse_expression(
    tokens: &mut Tokens,
    location: ExpressionLocation,
) -> Result<TypedExpression> {
    let expression = if let Token::Keyword(Fn) = tokens.peek().ok_or(())? {
        let location = FunctionLocation::AnonymousFunction { location };
        parse_function(tokens, location)
            .map(|function| Expression::LocalFunction { function })?
    } else {
        match tokens.take().ok_or(())? {
            Token::Comment { text } => Expression::Comment { text },
            Token::Identifier { name } => Expression::Identifier { name },
            Token::IntegerLiteral { value } => Expression::LiteralNumber {
                value: value.into(),
            },
            token => {
                panic!("Unexpected token: {token:?}");
            }
        }
    };

    let signature = parse_type_annotation(tokens).ok();

    let expression = TypedExpression {
        inner: expression,
        signature,
    };

    Ok(expression)
}

fn parse_type_annotation(tokens: &mut Tokens) -> Result<ConcreteSignature> {
    let Token::Punctuator(Introducer) = tokens.peek().ok_or(())? else {
        return Err(());
    };
    tokens.take().ok_or(())?;

    let signature = parse_signature(tokens)?;

    Ok(signature)
}

fn parse_signature(tokens: &mut Tokens) -> Result<ConcreteSignature> {
    match tokens.take().ok_or(())? {
        Token::Punctuator(Transformer) => {}
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    let mut outputs = Vec::new();

    let type_ = parse_type(tokens)?;
    outputs.push(type_);

    Ok(ConcreteSignature {
        inputs: vec![],
        outputs,
    })
}

fn parse_type(tokens: &mut Tokens) -> Result<Type> {
    let type_ = match tokens.take().ok_or(())? {
        Token::Identifier { name } => match name.as_str() {
            "Number" => Type::Number,
            type_ => panic!("Unknown type: `{type_}`"),
        },
        token => {
            panic!("Unexpected token: {token:?}");
        }
    };

    Ok(type_)
}

type Result<T> = result::Result<T, ()>;
