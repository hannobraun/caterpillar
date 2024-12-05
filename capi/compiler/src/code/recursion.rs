use std::collections::BTreeMap;

use super::{
    syntax::{Expression, FunctionLocation, MemberLocation},
    FunctionCalls, Functions, Index, OrderedFunctions,
};

/// # Tracks recursive expressions
///
/// There are two types of expressions that can be recursive:
///
/// - Calls to user-defined functions
/// - Local functions
///
/// A call to a user-defined function can be recursive in the following ways:
///
/// - They are calling the named function in which they are defined. This means
///   they could be defined in that named function directly, or indirectly in
///   any of its local child functions, or any of their children, recursively.
/// - They are calling another function that call the named function in which
///   they are defined. The call from that other function could be indirect,
///   through any number of mutually recursive functions.
///
/// Local functions can be recursive in the following ways:
///
/// - They contain a recursive call.
/// - They contain a recursive function.
///
/// This means all recursion is rooted in a recursive function call, but that
/// recursive call "infects" any local function that contains it.
///
/// ## Implementation Note
///
/// As of this writing, recursive local functions are not tracked yet. This is
/// subject to an ongoing transition.
#[derive(Debug)]
pub struct Recursion {
    inner: BTreeMap<MemberLocation, Index<FunctionLocation>>,
}

impl Recursion {
    /// # Find all recursive expressions
    pub fn find(
        function_calls: &FunctionCalls,
        functions: &Functions,
        ordered_functions: &OrderedFunctions,
    ) -> Self {
        let mut recursive_expressions = BTreeMap::new();

        for cluster in ordered_functions.clusters_from_leaves() {
            for function in cluster.functions(functions) {
                for branch in function.branches() {
                    for expression in branch.body() {
                        match expression.fragment {
                            Expression::Identifier { name: _ } => {
                                let Some(location) = function_calls
                                    .is_call_to_user_defined_function(
                                        &expression.location,
                                    )
                                else {
                                    continue;
                                };

                                if let Some(index) =
                                    cluster.find_function_by_location(location)
                                {
                                    recursive_expressions
                                        .insert(expression.location, index);
                                }
                            }
                            Expression::LocalFunction { function: _ } => {
                                let location = FunctionLocation::from(
                                    expression.location.clone(),
                                );

                                if let Some(index) =
                                    cluster.find_function_by_location(&location)
                                {
                                    recursive_expressions
                                        .insert(expression.location, index);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Self {
            inner: recursive_expressions,
        }
    }

    /// # Determine, if an expression is recursive
    ///
    /// If so, returns the index of the target function, within its cluster. The
    /// target function is defined as the following:
    ///
    /// - If the expression is a call, the index of the called function.
    /// - If the expression is a local function, the index of that local
    ///   function.
    ///
    /// This index is only valid within the context of the cluster that the
    /// expression is defined in.
    pub fn is_recursive_expression(
        &self,
        location: &MemberLocation,
    ) -> Option<Index<FunctionLocation>> {
        self.inner.get(location).copied()
    }
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    use crate::{
        code::{syntax::SyntaxTree, FunctionCalls, Tokens},
        host::NoHost,
        passes::order_functions_by_dependencies,
    };

    use super::Recursion;

    #[test]
    fn self_recursive_direct_call() {
        let (syntax_tree, recursion) = find_recursion(
            r"
                f: fn
                    \ ->
                        nop
                        f
                end
            ",
        );

        let (nop, f) = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&f).is_some());
    }

    #[test]
    fn self_recursive_indirect_call() {
        let (syntax_tree, recursion) = find_recursion(
            r"
                f: fn
                    \ ->
                        fn
                            \ ->
                                nop
                                f
                        end
                end
            ",
        );

        let (nop, f) = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .filter_map(|expression| expression.into_local_function())
            .next()
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&f).is_some());
    }

    #[test]
    fn mutually_recursive_direct_call() {
        let (syntax_tree, recursion) = find_recursion(
            r"
                f: fn
                    \ ->
                        nop
                        g
                end

                g: fn
                    \ ->
                        f
                end
            ",
        );

        let (nop, g) = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&g).is_some());
    }

    #[test]
    fn mutually_recursive_indirect_call() {
        let (syntax_tree, recursion) = find_recursion(
            r"
                f: fn
                    \ ->
                        fn
                            \ ->
                            nop
                            g
                        end
                end

                g: fn
                    \ ->
                        f
                end
            ",
        );

        let (nop, g) = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .filter_map(|expression| expression.into_local_function())
            .next()
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&g).is_some());
    }

    #[test]
    fn self_recursive_direct_local_function() {
        let (syntax_tree, recursion) = find_recursion(
            r"
                f: fn
                    \ ->
                        nop
                        fn
                            \ ->
                                f
                        end
                end
            ",
        );

        let (nop, function) = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&function).is_some());
    }

    #[test]
    fn self_recursive_indirect_local_function() {
        let (syntax_tree, recursion) = find_recursion(
            r"
                f: fn
                    \ ->
                        nop
                        fn
                            \ ->
                                fn
                                    \ ->
                                        f
                                end
                        end
                end
            ",
        );

        let (nop, function) = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&function).is_some());
    }

    #[test]
    fn mutually_recursive_direct_local_function() {
        let (syntax_tree, recursion) = find_recursion(
            r"
                f: fn
                    \ ->
                        nop
                        fn
                            \ ->
                                g
                        end
                end

                g: fn
                    \ ->
                        f
                end
            ",
        );

        let (nop, function) = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&function).is_some());
    }

    #[test]
    fn mutually_recursive_indirect_local_function() {
        let (syntax_tree, recursion) = find_recursion(
            r"
                f: fn
                    \ ->
                        nop
                        fn
                            \ ->
                                fn
                                    \ ->
                                        g
                                end
                        end
                end

                g: fn
                    \ ->
                        f
                end
            ",
        );

        let (nop, function) = syntax_tree
            .function_by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|expression| expression.location)
            .collect_tuple()
            .unwrap();

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&function).is_some());
    }

    fn find_recursion(input: &str) -> (SyntaxTree, Recursion) {
        let tokens = Tokens::tokenize(input);
        let syntax_tree = SyntaxTree::parse(tokens);
        let function_calls = FunctionCalls::resolve(&syntax_tree, &NoHost);
        let (functions, ordered_functions) =
            order_functions_by_dependencies(&syntax_tree, &function_calls);
        let recursion =
            Recursion::find(&function_calls, &functions, &ordered_functions);

        (syntax_tree, recursion)
    }
}
