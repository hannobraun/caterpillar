use crate::code::{
    tokens::{Token, Tokens},
    Branch, BranchLocation, Expression, ExpressionLocation, Function,
    FunctionLocation, Functions, Index, NamedFunction, Pattern,
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
pub fn parse(mut tokens: Tokens) -> Functions {
    let mut functions = Functions::default();

    loop {
        let index = functions.named.next_index();

        let Some(function) =
            parse_named_function(&mut tokens, index, &mut functions)
        else {
            break;
        };

        let actual_index = functions.named.push(function);
        assert_eq!(
            index, actual_index,
            "Function has a different index than was initially assumed.",
        );
    }

    functions
}

fn parse_named_function(
    tokens: &mut Tokens,
    index: Index<NamedFunction>,
    functions: &mut Functions,
) -> Option<NamedFunction> {
    let name = loop {
        if let Some(Token::Comment { .. }) = tokens.peek() {
            // Comments in the top-level context are currently ignored.
            tokens.take();
            continue;
        }

        match tokens.take()? {
            Token::FunctionName { name } => {
                break name;
            }
            token => {
                panic!("Unexpected token: {token:?}");
            }
        }
    };

    let location = FunctionLocation::NamedFunction { index };
    let function = parse_function(tokens, location, functions)?;

    Some(NamedFunction {
        name,
        inner: function,
    })
}

fn parse_function(
    tokens: &mut Tokens,
    location: FunctionLocation,
    functions: &mut Functions,
) -> Option<Function> {
    let mut function = Function::default();

    match tokens.take()? {
        Token::KeywordFn => {}
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    loop {
        let location = BranchLocation {
            parent: Box::new(location.clone()),
            index: function.branches.next_index(),
        };

        let Some(branch) = parse_branch(tokens, location, functions) else {
            break;
        };

        function.branches.push(branch);
    }

    match tokens.take()? {
        Token::KeywordEnd => {}
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    Some(function)
}

fn parse_branch(
    tokens: &mut Tokens,
    location: BranchLocation,
    functions: &mut Functions,
) -> Option<Branch> {
    match tokens.peek()? {
        Token::BranchStart => {
            tokens.take();
        }
        Token::KeywordEnd => {
            return None;
        }
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }

    let mut branch = Branch::default();

    parse_branch_parameters(tokens, &mut branch);
    parse_branch_body(tokens, &mut branch, location, functions)?;

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
            Token::Delimiter => {
                // If we have a delimiter, then we're good here. Next loop
                // iteration, we'll either parse the next parameter, or if it
                // was the last one, find the start of the branch body.
                continue;
            }
            Token::BranchBodyStart => {
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
        Token::BranchBodyStart => None,
        token => {
            panic!("Unexpected token: {token:?}");
        }
    }
}

fn parse_branch_body(
    tokens: &mut Tokens,
    branch: &mut Branch,
    location: BranchLocation,
    functions: &mut Functions,
) -> Option<()> {
    while let Some(token) = tokens.peek() {
        match token {
            Token::KeywordFn => {
                let location = ExpressionLocation {
                    parent: Box::new(location.clone()),
                    index: branch.body.next_index(),
                };

                if let Some(function) = parse_function(
                    tokens,
                    FunctionLocation::AnonymousFunction {
                        location: location.clone(),
                    },
                    functions,
                ) {
                    branch.body.push(Expression::LocalFunction);
                    functions.anonymous.insert(location, function);
                }
            }
            Token::BranchStart | Token::KeywordEnd => {
                break;
            }
            _ => match tokens.take()? {
                Token::Comment { text } => {
                    branch.body.push(Expression::Comment { text });
                }
                Token::Identifier { name } => {
                    branch.body.push(Expression::Identifier { name });
                }
                Token::IntegerLiteral { value } => {
                    branch.body.push(Expression::LiteralNumber {
                        value: value.into(),
                    });
                }
                token => {
                    panic!("Unexpected token: {token:?}");
                }
            },
        }
    }

    Some(())
}
