use std::collections::BTreeSet;

use super::{
    Branch, Expression, ExpressionLocation, Function, FunctionLocation,
    Functions, Located, Pattern,
};

/// # Tracks bindings
///
/// A binding is an identifier that refers to a value that was previously bound
/// to a name.
#[derive(Debug)]
pub struct Bindings {
    bindings: BTreeSet<ExpressionLocation>,
}

impl Bindings {
    /// # Resolve all bindings
    pub fn resolve_bindings(functions: &Functions) -> Self {
        let mut bindings = BTreeSet::new();
        resolve_bindings(functions, &mut bindings);
        Self { bindings }
    }

    /// # Determine, if the expression at the given location is a binding
    pub fn is_binding(&self, location: &ExpressionLocation) -> bool {
        self.bindings.contains(location)
    }
}

fn resolve_bindings(
    functions: &Functions,
    bindings: &mut BTreeSet<ExpressionLocation>,
) {
    let mut scopes = Scopes::new();

    for function in functions.all_functions() {
        resolve_bindings_in_function(
            function,
            functions,
            &mut scopes,
            bindings,
        );
    }
}

fn resolve_bindings_in_function(
    function: Located<&Function>,
    functions: &Functions,
    scopes: &mut Scopes,
    bindings: &mut BTreeSet<ExpressionLocation>,
) {
    for branch in function.branches() {
        resolve_bindings_in_branch(branch, functions, scopes, bindings);
    }
}

fn resolve_bindings_in_branch(
    branch: Located<&Branch>,
    functions: &Functions,
    scopes: &mut Scopes,
    bindings: &mut BTreeSet<ExpressionLocation>,
) {
    scopes.push(
        branch
            .parameters
            .clone()
            .into_iter()
            .filter_map(|pattern| {
                if let Pattern::Identifier { name } = pattern {
                    Some(name)
                } else {
                    None
                }
            })
            .collect(),
    );

    for expression in branch.body() {
        match expression.fragment {
            Expression::UnresolvedIdentifier {
                name,
                is_known_to_be_call_to_user_defined_function: _,
            } => {
                let is_known_binding =
                    scopes.iter().any(|scope| scope.contains(name));

                if is_known_binding {
                    bindings.insert(expression.location);
                }
            }
            Expression::UnresolvedLocalFunction => {
                let location = FunctionLocation::from(expression.location);
                let function = functions
                    .by_location(&location)
                    .expect("Function referred to from branch must exist.");

                resolve_bindings_in_function(
                    function, functions, scopes, bindings,
                );
            }
            _ => {}
        }
    }

    scopes.pop();
}

type Scopes = Vec<BindingsInScope>;
type BindingsInScope = BTreeSet<String>;

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::code::{
        syntax::parse, Expression, FunctionLocation, Functions, Tokens,
    };

    use super::Bindings;

    #[test]
    fn resolve_parameter_of_function() {
        // An identifier with the same name as a function's parameter should be
        // resolved as a binding.

        let (functions, bindings) = resolve_bindings(
            r"
                f: fn
                    \ parameter ->
                        parameter
                        no_parameter
                end
            ",
        );

        let (parameter, no_parameter) = functions
            .named
            .by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(bindings.is_binding(&parameter));
        assert!(!bindings.is_binding(&no_parameter));
    }

    #[test]
    fn resolve_parameter_of_parent_function() {
        // An identifier with the same name as the parameter of a parent
        // function should be resolved as a binding.

        let (functions, bindings) = resolve_bindings(
            r"
                f: fn
                    \ parameter ->
                        fn
                            \ ->
                                parameter
                                no_parameter
                        end
                end
            ",
        );

        let function = functions
            .named
            .by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .filter_map(|expression| {
                if let Expression::UnresolvedLocalFunction = expression.fragment
                {
                    let location = FunctionLocation::from(expression.location);
                    let function = functions.by_location(&location);
                    Some(function)
                } else {
                    None
                }
            })
            .flatten()
            .next()
            .unwrap();
        let (parameter, no_parameter) = function
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(bindings.is_binding(&parameter));
        assert!(!bindings.is_binding(&no_parameter));
    }

    #[test]
    fn do_not_resolve_parameter_of_child_function() {
        // Identifiers that share a name with a parameter of a child function
        // should not be resolved as bindings.

        let (functions, bindings) = resolve_bindings(
            r"
                f: fn
                    \ ->
                        fn
                            \ child_parameter ->
                        end
                    
                    child_parameter
                end
            ",
        );

        let child_parameter = functions
            .named
            .by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .nth(1)
            .unwrap();

        assert!(!bindings.is_binding(&child_parameter));
    }

    fn resolve_bindings(input: &str) -> (Functions, Bindings) {
        let tokens = Tokens::from_input(input);
        let functions = parse(tokens);
        let bindings = Bindings::resolve_bindings(&functions);

        (functions, bindings)
    }
}
