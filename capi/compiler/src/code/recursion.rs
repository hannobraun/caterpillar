use std::collections::BTreeMap;

use super::{
    Expression, ExpressionLocation, FunctionLocation, Functions, Index,
    OrderedFunctions,
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
    inner: BTreeMap<ExpressionLocation, Index<FunctionLocation>>,
}

impl Recursion {
    /// # Find all recursive expressions
    pub fn find(
        functions: &Functions,
        ordered_functions: &OrderedFunctions,
    ) -> Self {
        let mut recursive_expressions = BTreeMap::new();

        for cluster in ordered_functions.clusters_from_leaves() {
            for function in cluster.functions(functions) {
                for branch in function.branches() {
                    for expression in branch.body() {
                        match expression.fragment {
                            Expression::Identifier { name } => {
                                if let Some(index) = cluster
                                    .find_function_by_name(name, functions)
                                {
                                    recursive_expressions
                                        .insert(expression.location, index);
                                }
                            }
                            Expression::LocalFunction => {
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
        location: &ExpressionLocation,
    ) -> Option<Index<FunctionLocation>> {
        self.inner.get(location).copied()
    }
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    use crate::{
        code::{syntax::parse, FunctionCalls, Functions, Tokens},
        host::NoHost,
        passes::order_functions_by_dependencies,
    };

    use super::Recursion;

    #[test]
    fn self_recursive_direct_call() {
        let (functions, recursion) = find_recursion(
            r"
                f: fn
                    \ ->
                        nop
                        f
                end
            ",
        );

        let (nop, f) = functions
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

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&f).is_some());
    }

    #[test]
    fn self_recursive_indirect_call() {
        let (functions, recursion) = find_recursion(
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

        let (nop, f) = functions
            .named
            .by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .filter_map(|expression| {
                let location = expression.to_function_location()?;
                let function = functions.by_location(&location)?;
                Some(function)
            })
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
        let (functions, recursion) = find_recursion(
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

        let (nop, g) = functions
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

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&g).is_some());
    }

    #[test]
    fn mutually_recursive_indirect_call() {
        let (functions, recursion) = find_recursion(
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

        let (nop, g) = functions
            .named
            .by_name("f")
            .unwrap()
            .into_located_function()
            .find_single_branch()
            .unwrap()
            .body()
            .filter_map(|expression| {
                let location = expression.to_function_location()?;
                let function = functions.by_location(&location)?;
                Some(function)
            })
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
        let (functions, recursion) = find_recursion(
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

        let (nop, function) = functions
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

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&function).is_some());
    }

    #[test]
    fn self_recursive_indirect_local_function() {
        let (functions, recursion) = find_recursion(
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

        let (nop, function) = functions
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

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&function).is_some());
    }

    #[test]
    fn mutually_recursive_direct_local_function() {
        let (functions, recursion) = find_recursion(
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

        let (nop, function) = functions
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

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&function).is_some());
    }

    #[test]
    fn mutually_recursive_indirect_local_function() {
        let (functions, recursion) = find_recursion(
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

        let (nop, function) = functions
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

        assert!(recursion.is_recursive_expression(&nop).is_none());
        assert!(recursion.is_recursive_expression(&function).is_some());
    }

    fn find_recursion(input: &str) -> (Functions, Recursion) {
        let tokens = Tokens::tokenize(input);
        let functions = parse(tokens);
        let function_calls = FunctionCalls::resolve(&functions, &NoHost);
        let ordered_functions =
            order_functions_by_dependencies(&functions, &function_calls);
        let recursion = Recursion::find(&functions, &ordered_functions);

        (functions, recursion)
    }
}