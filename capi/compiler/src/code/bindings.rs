use std::collections::BTreeSet;

use crate::code::{
    AnonymousFunctions, Branch, Expression, Function, FunctionLocation,
    Functions, Located, Pattern,
};

pub fn resolve_bindings(functions: &mut Functions) {
    let mut scopes = Scopes::new();

    for function in functions.named.iter_mut() {
        resolve_bindings_in_function(
            function.into_located_function_mut(),
            &mut scopes,
            &mut functions.anonymous,
        );
    }
}

fn resolve_bindings_in_function(
    function: Located<&mut Function>,
    scopes: &mut Scopes,
    anonymous_functions: &mut AnonymousFunctions,
) {
    let (branches, environment) = function.destructure();

    for branch in branches {
        scopes.push(
            branch
                .parameters
                .clone()
                .into_iter()
                .filter_map(|pattern| match pattern {
                    Pattern::Identifier { name } => Some(name),
                    Pattern::Literal { .. } => {
                        // The scope is used to resolve identifiers against
                        // known bindings. Literal patterns don't create
                        // bindings, as their value is only used to select
                        // the function to be called.
                        None
                    }
                })
                .collect(),
        );

        resolve_bindings_in_branch(
            branch,
            scopes,
            environment,
            anonymous_functions,
        );
    }
}

fn resolve_bindings_in_branch(
    branch: Located<&mut Branch>,
    scopes: &mut Scopes,
    environment: &mut BTreeSet<String>,
    anonymous_functions: &mut AnonymousFunctions,
) {
    let (body, _) = branch.destructure();

    for expression in body {
        match expression.fragment {
            Expression::UnresolvedIdentifier {
                name,
                is_known_to_be_call_to_user_defined_function: _,
            } => {
                // The way this is written, definitions can silently shadow each
                // other in a defined order. This is undesirable.
                //
                // There should at least be a warning, if such shadowing
                // shouldn't be forbidden outright.
                if scopes.iter().any(|bindings| bindings.contains(name)) {
                    if let Some(bindings) = scopes.last() {
                        if !bindings.contains(name) {
                            environment.insert(name.clone());
                        }
                    }

                    *expression.fragment =
                        Expression::Binding { name: name.clone() }
                }
            }
            Expression::UnresolvedLocalFunction => {
                // Need to remove this and put it back below, since we need to
                // mutably borrow in between.
                let mut function =
                    anonymous_functions.remove(&expression.location).expect(
                        "The current expression is an anonymous function, or \
                        we wouldn't be in this `match` arm. It must be tracked \
                        in `anonymous_functions` under this location.\n\
                        \n\
                        Since this compiler pass is going through the code \
                        lexically, and locations are unique, we should not \
                        encounter this location again. Hence, that we remove \
                        an anonymous function from `anonymous_functions` here \
                        should have no bearing.",
                    );

                resolve_bindings_in_function(
                    Located {
                        fragment: &mut function,
                        location: FunctionLocation::AnonymousFunction {
                            location: expression.location.clone(),
                        },
                    },
                    scopes,
                    anonymous_functions,
                );

                for name in &function.environment {
                    // If the child function we just resolved identifiers for
                    // captures something from its environment, and the current
                    // scope doesn't already have that, then it needs to capture
                    // it from its environment likewise.
                    if let Some(bindings) = scopes.last() {
                        if !bindings.contains(name) {
                            environment.insert(name.clone());
                        }
                    }
                }

                anonymous_functions.insert(expression.location, function);
            }
            _ => {}
        }
    }

    scopes.pop();
}

type Scopes = Vec<Bindings>;
type Bindings = BTreeSet<String>;

#[cfg(test)]
mod tests {
    use crate::code::{syntax::parse, Branch, Expression, Tokens};

    #[test]
    fn do_not_resolve_binding_from_child_scope() {
        // Bindings that are defined in a scope that is a lexical child of the
        // current scope, should not be resolved.

        let mut functions = resolve_bindings(
            r"
                f: fn
                    \ ->
                        0
                        fn
                            \ value ->
                        end
                        value
                end
            ",
        );

        assert_eq!(
            functions
                .remove(0)
                .body
                .last_key_value()
                .map(|(_, expression)| expression),
            Some(&Expression::UnresolvedIdentifier {
                name: String::from("value"),
                is_known_to_be_call_to_user_defined_function: false,
            })
        );
    }

    fn resolve_bindings(input: &str) -> Vec<Branch> {
        let tokens = Tokens::from_input(input);
        let mut functions = parse(tokens);
        super::resolve_bindings(&mut functions);

        functions
            .named
            .into_iter()
            .flat_map(|function| function.fragment.inner.branches.into_values())
            .collect()
    }
}
