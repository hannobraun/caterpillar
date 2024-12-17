use crate::code::{
    syntax::{Function, FunctionLocation, Located, NamedFunction, SyntaxTree},
    FunctionCalls, Index,
};

use super::resolve::resolve_dependencies;

/// # Tracks the dependencies between functions
///
/// A dependency can take two forms:
///
/// - If any function `f` calls a named function `g`, then `f` depends on `g`.
/// - If any function `f` has a local function `g`, then `f` depends on `g`.
///
/// Dependencies form a graph, not a tree, since:
///
/// - Named functions can call themselves.
/// - Named functions can call each other.
/// - Local functions can call their top-level parent function (which is always
///   named), or another named function that (directly or indirectly) calls
///   their top-level parent function.
///
/// Named functions that call themselves are designated as "self-recursive".
/// Groups of mutually dependent functions are designated as "mutually
/// recursive".
///
/// All functions are grouped into [`DependencyCluster`]s. Either by themselves,
/// if they are not recursive or self-recursive, or together with all the
/// functions in their mutually dependent group.
///
/// Using graph theory jargon, dependency clusters are a "condensation" of the
/// original dependency graph.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Dependencies {
    clusters: Vec<DependencyCluster>,
}

impl Dependencies {
    /// # Resolve the dependencies
    pub fn resolve(
        syntax_tree: &SyntaxTree,
        function_calls: &FunctionCalls,
    ) -> Self {
        let clusters = resolve_dependencies(syntax_tree, function_calls)
            .into_iter()
            .map(|functions| DependencyCluster { functions })
            .collect();
        Self { clusters }
    }

    /// # Iterate over the dependency clusters
    ///
    /// Any cluster that the iterator yields only has dependencies on functions
    /// in the cluster itself, or functions in clusters that it yielded before.
    pub fn clusters(&self) -> impl Iterator<Item = &DependencyCluster> {
        self.clusters.iter().rev()
    }

    /// # Iterate over all functions
    ///
    /// Any function that the iterator yields only has non-recursive
    /// dependencies on functions that it yielded before.
    ///
    /// If recursive dependencies are relevant, use [`Dependencies::clusters`].
    ///
    /// ## Panics
    ///
    /// Panics, if a function tracked in this instance of [`Dependencies`] can
    /// not be found in the provided [`SyntaxTree`]. This should only happen, if
    /// you pass a different [`SyntaxTree`] to this function than you previously
    /// passed to [`Dependencies::resolve`].
    pub fn functions<'r>(
        &'r self,
        syntax_tree: &'r SyntaxTree,
    ) -> impl Iterator<Item = Located<&'r Function>> {
        self.clusters()
            .flat_map(|cluster| cluster.functions(syntax_tree))
    }

    /// # Find the cluster containing a specific named function
    pub fn find_cluster_by_named_function(
        &self,
        index: &Index<NamedFunction>,
    ) -> Option<&DependencyCluster> {
        self.clusters.iter().find(|cluster| {
            cluster.functions.iter().any(|location| {
                *location == FunctionLocation::Named { index: *index }
            })
        })
    }
}

/// # A cluster of mutually dependent functions, or a single function
///
/// See [`Dependencies`] for more information.
#[derive(
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct DependencyCluster {
    functions: Vec<FunctionLocation>,
}

impl DependencyCluster {
    /// # Iterate over the functions in the cluster
    ///
    /// ## Panics
    ///
    /// Panics, if a function tracked in this instance of [`DependencyCluster`]
    /// can not be found in the provided [`SyntaxTree`]. This should only
    /// happen, if you pass a different [`SyntaxTree`] to this function than you
    /// previously passed to [`Dependencies::resolve`].
    pub fn functions<'r>(
        &'r self,
        syntax_tree: &'r SyntaxTree,
    ) -> impl Iterator<Item = Located<&'r Function>> {
        self.functions.iter().map(|location| {
            let Some(function) = syntax_tree.function_by_location(location)
            else {
                panic!(
                    "This function expects to find all tracked locations in \
                    the provided `SyntaxTree`."
                );
            };

            function
        })
    }

    /// # Indicate whether the cluster contains the given function
    pub fn contains_function(&self, location: &FunctionLocation) -> bool {
        self.functions.iter().any(|l| location == l)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        code::{
            syntax::{FunctionLocation, SyntaxTree},
            Dependencies, FunctionCalls, Tokens,
        },
        host::NoHost,
    };

    #[test]
    fn no_recursion() {
        let (syntax_tree, dependencies) = resolve_dependencies(
            r"
                f: fn
                    \ ->
                        g
                end

                g: fn
                    \ ->
                        nop
                end
            ",
        );

        let f = syntax_tree.function_by_name("f").unwrap().location();
        let g = syntax_tree.function_by_name("g").unwrap().location();

        assert_eq!(destructure(dependencies, &syntax_tree), [[g], [f]]);
    }

    #[test]
    fn self_recursion() {
        let (syntax_tree, dependencies) = resolve_dependencies(
            r"
                f: fn
                    \ ->
                        g
                end

                g: fn
                    \ ->
                        g
                end
            ",
        );

        let f = syntax_tree.function_by_name("f").unwrap().location();
        let g = syntax_tree.function_by_name("g").unwrap().location();

        assert_eq!(destructure(dependencies, &syntax_tree), [[g], [f]]);
    }

    #[test]
    fn mutual_recursion() {
        let (syntax_tree, dependencies) = resolve_dependencies(
            r"
                f: fn
                    \ ->
                        g
                end

                g: fn
                    \ ->
                        h
                end

                h: fn
                    \ ->
                        g
                end
            ",
        );

        let f = syntax_tree.function_by_name("f").unwrap().location();
        let g = syntax_tree.function_by_name("g").unwrap().location();
        let h = syntax_tree.function_by_name("h").unwrap().location();

        assert_eq!(
            destructure(dependencies, &syntax_tree),
            [vec![g, h], vec![f]],
        );
    }

    #[test]
    fn sort_clusters_by_call_graph() {
        let (syntax_tree, dependencies) = resolve_dependencies(
            r"
                f: fn
                    \ ->
                        # Call a function that comes is placed after this one in
                        # the source code.
                        h
                end

                g: fn
                    \ ->
                        nop
                end

                h: fn
                    \ ->
                        # And for some variety, call a function that is placed
                        # before.
                        g
                end
            ",
        );

        let f = syntax_tree.function_by_name("f").unwrap().location();
        let g = syntax_tree.function_by_name("g").unwrap().location();
        let h = syntax_tree.function_by_name("h").unwrap().location();

        assert_eq!(destructure(dependencies, &syntax_tree), [[g], [h], [f]]);
    }

    #[test]
    fn consider_anonymous_functions_in_call_graph() {
        let (syntax_tree, dependencies) = resolve_dependencies(
            r"
                f: fn
                    \ ->
                        fn
                            \ ->
                                # Call a function that is placed after this one
                                # in the source code.
                                h
                        end
                end

                g: fn
                    \ ->
                        nop
                end

                h: fn
                    \ ->
                        fn
                            \ ->
                                # And for some variety, call a function that is
                                # placed before.
                                g
                        end
                end
            ",
        );

        let [(f, f_a), (h, h_a)] = ["f", "h"].map(|name| {
            let named = syntax_tree.function_by_name(name).unwrap();
            let anonymous = named
                .into_located_function()
                .find_single_branch()
                .unwrap()
                .expressions()
                .next()
                .unwrap()
                .into_local_function()
                .map(|function| function.location)
                .unwrap();

            (named.location(), anonymous)
        });
        let g = syntax_tree.function_by_name("g").unwrap().location();

        assert_eq!(
            destructure(dependencies, &syntax_tree),
            [[g], [h_a], [h], [f_a], [f]],
        );
    }

    fn resolve_dependencies(input: &str) -> (SyntaxTree, Dependencies) {
        let tokens = Tokens::tokenize(input);
        let syntax_tree = SyntaxTree::parse(tokens);
        let function_calls = FunctionCalls::resolve(&syntax_tree, &NoHost);
        let dependencies = Dependencies::resolve(&syntax_tree, &function_calls);

        (syntax_tree, dependencies)
    }

    fn destructure(
        dependencies: Dependencies,
        syntax_tree: &SyntaxTree,
    ) -> Vec<Vec<FunctionLocation>> {
        dependencies
            .clusters()
            .map(|cluster| {
                cluster
                    .functions(syntax_tree)
                    .map(|function| function.location)
                    .collect()
            })
            .collect()
    }
}
