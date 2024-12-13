use std::collections::BTreeMap;

use crate::code::syntax::{
    Branch, BranchLocation, Expression, Function, FunctionLocation, Located,
    MemberLocation, SyntaxTree,
};

/// # Tracks bindings
///
/// A binding is an identifier that refers to a value that was previously bound
/// to a name.
#[derive(Debug)]
pub struct Bindings {
    bindings: BindingsMap,
    environments: EnvironmentsMap,
}

impl Bindings {
    /// # Resolve all bindings
    pub fn resolve(syntax_tree: &SyntaxTree) -> Self {
        let mut bindings = BTreeMap::new();
        let mut environments = BTreeMap::new();

        resolve_bindings(syntax_tree, &mut bindings, &mut environments);

        Self {
            bindings,
            environments,
        }
    }

    /// # Determine, if the expression at the given location is a binding
    pub fn is_binding(&self, location: &MemberLocation) -> Option<&Binding> {
        self.bindings.get(location)
    }

    /// # Access the environment of the function at the provided location
    pub fn environment_of(&self, location: &FunctionLocation) -> &Environment {
        static EMPTY: Environment = Environment::new();
        self.environments.get(location).unwrap_or(&EMPTY)
    }
}

type BindingsMap = BTreeMap<MemberLocation, Binding>;
type EnvironmentsMap = BTreeMap<FunctionLocation, Environment>;

/// # A binding
///
/// A binding is a value that has been bound to a name, locally within a branch.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Binding {
    /// # The index of the identifier parameter that defines the binding
    ///
    /// An identifier index is the 0-based index of a parameter within a
    /// branch's list of parameters, only counting parameters that bind to an
    /// identifier within the branch.
    ///
    /// Parameters are patterns that could bind a value to an identifier that is
    /// then available within the branch, or they could just match an argument,
    /// but not make any value available in the branch.
    ///
    /// This index is required to keep track of bindings on the stack.
    /// Parameters that do not bind to an identifier are not relevant for that,
    /// since they do not create bindings.
    pub identifier_index: u32,

    /// # The branch in which the binding is defined
    pub branch: BranchLocation,
}

/// # The environment of a function
///
/// The environment of a function is the set of bindings it accesses, that are
/// not its own parameters.
pub type Environment = BTreeMap<String, Binding>;

fn resolve_bindings(
    syntax_tree: &SyntaxTree,
    bindings: &mut BindingsMap,
    environments: &mut EnvironmentsMap,
) {
    let mut scopes = Scopes::new();

    for function in syntax_tree.named_functions() {
        resolve_bindings_in_function(
            function.into_located_function(),
            &mut scopes,
            bindings,
            environments,
        );
    }
}

fn resolve_bindings_in_function(
    function: Located<&Function>,
    scopes: &mut Scopes,
    bindings: &mut BindingsMap,
    environments: &mut EnvironmentsMap,
) -> Environment {
    let location = function.location.clone();
    let mut environment = Environment::new();

    for branch in function.branches() {
        resolve_bindings_in_branch(
            branch,
            scopes,
            bindings,
            &mut environment,
            environments,
        );
    }

    let overwritten_value = environments.insert(location, environment.clone());
    assert!(
        overwritten_value.is_none(),
        "Every function should be processed only once."
    );

    environment
}

fn resolve_bindings_in_branch(
    branch: Located<&Branch>,
    scopes: &mut Scopes,
    bindings: &mut BindingsMap,
    environment: &mut Environment,
    environments: &mut EnvironmentsMap,
) {
    scopes.push(branch.bindings().collect());

    for expression in branch.expressions() {
        match expression.fragment {
            Expression::Identifier { name } => {
                let binding = scopes.iter().rev().find_map(|scope| {
                    scope
                        .iter()
                        .find_map(|(n, binding)| (n == name).then_some(binding))
                });

                if let Some(binding) = binding {
                    bindings.insert(expression.location, binding.clone());

                    if let Some(scope) = scopes.last() {
                        if !scope.contains_key(name) {
                            // The binding is not known in the current scope,
                            // which means it comes from a parent scope.
                            environment.insert(name.clone(), binding.clone());
                        }
                    }
                }
            }
            Expression::LocalFunction { function } => {
                let function = Located {
                    fragment: function,
                    location: FunctionLocation::from(expression.location),
                };

                let child_environment = resolve_bindings_in_function(
                    function,
                    scopes,
                    bindings,
                    environments,
                );

                for (name, binding) in child_environment {
                    if let Some(bindings) = scopes.last() {
                        if !bindings.contains_key(&name) {
                            // The child function that we just resolved bindings
                            // in has a function in its environment that is not
                            // known in the current scope.
                            //
                            // This means it must come from this function's
                            // parent scopes, and must be added to this
                            // environment too.
                            environment.insert(name.clone(), binding);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    scopes.pop();
}

type Scopes = Vec<BindingsInScope>;
type BindingsInScope = BTreeMap<String, Binding>;

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::code::{
        syntax::{Expression, FunctionLocation, SyntaxTree},
        Tokens,
    };

    use super::Bindings;

    #[test]
    fn resolve_parameter_of_function() {
        // An identifier with the same name as a function's parameter should be
        // resolved as a binding.

        let (syntax_tree, bindings) = resolve_bindings(
            r"
                f: fn
                    \ parameter ->
                        parameter
                        no_parameter
                end
            ",
        );

        let (parameter, no_parameter) = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .expressions()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(bindings.is_binding(&parameter).is_some());
        assert!(bindings.is_binding(&no_parameter).is_none());
    }

    #[test]
    fn resolve_parameter_of_parent_function() {
        // An identifier with the same name as the parameter of a parent
        // function should be resolved as a binding.

        let (syntax_tree, bindings) = resolve_bindings(
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

        let function = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .expressions()
            .filter_map(|expression| expression.into_local_function())
            .next()
            .unwrap()
            .cloned();
        let (parameter, no_parameter) = function
            .as_ref()
            .find_single_branch()
            .unwrap()
            .expressions()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(bindings.is_binding(&parameter).is_some());
        assert!(bindings.is_binding(&no_parameter).is_none());

        assert!(bindings
            .environment_of(&function.location)
            .contains_key("parameter"));
    }

    #[test]
    fn resolve_parameter_that_shadows_parent_parameter() {
        // If a function parameter has the same name as the parameter of a
        // parent function, the closer parameter should be resolved.

        let (syntax_tree, bindings) = resolve_bindings(
            r"
                f: fn
                    \ parameter ->
                        fn
                            \ parameter ->
                                parameter
                        end
                end
            ",
        );

        let function = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .expressions()
            .filter_map(|expression| expression.into_local_function())
            .next()
            .unwrap()
            .cloned();
        let function = function.as_ref();
        let branch = function.find_single_branch().unwrap();
        let parameter = branch
            .expressions()
            .map(|expression| expression.location)
            .next()
            .unwrap();

        if let Some(binding) = bindings.is_binding(&parameter) {
            assert_eq!(binding.branch, branch.location);
        } else {
            panic!("Expected identifier to be a binding.");
        }
        assert!(bindings.is_binding(&parameter).is_some());
    }

    #[test]
    fn do_not_resolve_parameter_of_child_function() {
        // Identifiers that share a name with a parameter of a child function
        // should not be resolved as bindings.

        let (syntax_tree, bindings) = resolve_bindings(
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

        let child_parameter = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .expressions()
            .map(|expression| expression.location)
            .nth(1)
            .unwrap();

        assert!(bindings.is_binding(&child_parameter).is_none());
    }

    #[test]
    fn track_indirect_bindings_in_environment() {
        // If a function doesn't access a binding from a parent scope itself,
        // but one of its child functions does, the binding still needs to be
        // part of the function's environment.

        let (syntax_tree, bindings) = resolve_bindings(
            r"
                f: fn
                    \ binding ->
                        fn
                            \ ->
                                fn
                                    \ ->
                                        binding
                                end
                        end
                        
                end
            ",
        );

        let function = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .expressions()
            .filter_map(|expression| {
                if let Expression::LocalFunction { function: _ } =
                    expression.fragment
                {
                    let location = FunctionLocation::from(expression.location);
                    Some(location)
                } else {
                    None
                }
            })
            .next()
            .unwrap();

        assert!(bindings.environment_of(&function).contains_key("binding"));
    }

    fn resolve_bindings(input: &str) -> (SyntaxTree, Bindings) {
        let tokens = Tokens::tokenize(input);
        let syntax_tree = SyntaxTree::parse(tokens);
        let bindings = Bindings::resolve(&syntax_tree);

        (syntax_tree, bindings)
    }
}
