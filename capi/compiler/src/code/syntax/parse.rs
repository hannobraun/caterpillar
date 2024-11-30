use crate::code::{
    tokens::{Keyword::*, Punctuator::*, Token, Tokens},
    Branch, BranchLocation, Expression, ExpressionLocation, Function,
    FunctionLocation, Index, IndexMap, NamedFunction, Pattern, Type,
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

        let Some(function) = parse_named_function(&mut tokens, index) else {
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
) -> Option<NamedFunction> {
    let name = parse_function_name(tokens)?;

    let location = FunctionLocation::NamedFunction { index };
    let function = parse_function(tokens, location)?;

    Some(NamedFunction {
        name,
        inner: function,
    })
}

fn parse_function_name(tokens: &mut Tokens) -> Option<String> {
    let name = loop {
        if let Some(Token::Comment { .. }) = tokens.peek() {
            // Comments in the top-level context are currently ignored.
            tokens.take();
            continue;
        }

        match tokens.take()? {
            Token::Identifier { name } => {
                break name;
            }
            token => {
                panic!("Unexpected token: {token:?}");
            }
        }
    };

    match tokens.take()? {
        Token::Punctuator(Introducer) => {}
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    Some(name)
}

fn parse_function(
    tokens: &mut Tokens,
    location: FunctionLocation,
) -> Option<Function> {
    let mut function = Function::default();

    match tokens.take()? {
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

        let Some(branch) = parse_branch(tokens, location) else {
            break;
        };

        function.branches.push(branch);
    }

    match tokens.take()? {
        Token::Keyword(End) => {}
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    Some(function)
}

fn parse_branch(
    tokens: &mut Tokens,
    location: BranchLocation,
) -> Option<Branch> {
    match tokens.peek()? {
        Token::Punctuator(BranchStart) => {
            tokens.take();
        }
        Token::Keyword(End) => {
            return None;
        }
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    let mut branch = Branch::default();

    parse_branch_parameters(tokens, &mut branch);
    parse_branch_body(tokens, &mut branch, location)?;

    Some(branch)
}

fn parse_branch_parameters(tokens: &mut Tokens, branch: &mut Branch) {
    loop {
        let Some(token) = tokens.take() else {
            break;
        };

        match parse_branch_parameter(token) {
            Some(pattern) => {
                branch.parameters.push(pattern);
            }
            None => {
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
            Token::Punctuator(BranchBodyStart) => {
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

fn parse_branch_parameter(token: Token) -> Option<Pattern> {
    match token {
        Token::Identifier { name } => Some(Pattern::Identifier { name }),
        Token::IntegerLiteral { value } => Some(Pattern::Literal {
            value: value.into(),
        }),
        Token::Punctuator(BranchBodyStart) => None,
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }
}

fn parse_branch_body(
    tokens: &mut Tokens,
    branch: &mut Branch,
    location: BranchLocation,
) -> Option<()> {
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

    Some(())
}

fn parse_expression(
    tokens: &mut Tokens,
    location: ExpressionLocation,
) -> Option<TypedExpression> {
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
                panic!("Unexpected token: {token:?}");
            }
        }
    };

    let type_ = parse_type_annotation(tokens);

    let expression = TypedExpression {
        inner: expression,
        type_,
    };

    Some(expression)
}

fn parse_type_annotation(_: &mut Tokens) -> Option<Type> {
    None
}
