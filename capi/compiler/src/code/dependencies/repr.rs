use crate::code::{
    syntax::{
        Branch, BranchLocation, Function, FunctionLocation, Located,
        NamedFunction, SyntaxTree,
    },
    FunctionCalls, Index,
};

use super::resolve::{
    resolve_branch_dependencies, resolve_function_dependencies,
};

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
        let clusters =
            resolve_function_dependencies(syntax_tree, function_calls)
                .into_iter()
                .map(|functions| {
                    let branches = resolve_branch_dependencies(
                        &functions,
                        syntax_tree,
                        function_calls,
                    );

                    DependencyCluster {
                        functions,
                        branches,
                    }
                })
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
    branches: Vec<BranchLocation>,
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

    /// # Iterate over the branches in the cluster, sorted by dependencies
    ///
    /// ## Panics
    ///
    /// Panics, if a branch tracked in this instance of [`DependencyCluster`]
    /// can not be found in the provided [`SyntaxTree`]. This should only
    /// happen, if you pass a different [`SyntaxTree`] to this function than you
    /// previously passed to [`Dependencies::resolve`].
    pub fn branches<'r>(
        &'r self,
        syntax_tree: &'r SyntaxTree,
    ) -> impl Iterator<Item = Located<&'r Branch>> {
        self.branches.iter().map(|location| {
            let Some(branch) = syntax_tree.branch_by_location(location) else {
                panic!(
                    "This function expects to find all tracked locations in \
                    the provided `SyntaxTree`."
                );
            };

            branch
        })
    }

    /// # Indicate whether the cluster contains the given function
    pub fn contains_function(&self, location: &FunctionLocation) -> bool {
        self.functions.iter().any(|l| l == location)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, fmt::Write};

    use itertools::Itertools;

    use crate::{
        code::{
            syntax::{FunctionLocation, SyntaxTree},
            Dependencies, FunctionCalls, Tokens,
        },
        host::NoHost,
    };

    #[test]
    fn function_dependencies_without_recursion() {
        let f = r"
            f: fn
                \ ->
                    g
                end
            end
        ";
        let g = r"
            g: fn
                \ ->
                    nop
                end
            end
        ";

        for [a, b] in [[f, g], [g, f]] {
            let (syntax_tree, dependencies) = resolve_dependencies(&format!(
                "
                    {a}
                    {b}
                "
            ));

            let [f, g] = ["f", "g"].map(|name| {
                syntax_tree.function_by_name(name).unwrap().location()
            });

            assert_eq!(
                dependencies_by_function(&dependencies, &syntax_tree),
                [[g], [f]]
            );
        }
    }

    #[test]
    fn function_dependencies_in_the_presence_of_self_recursion() {
        let f = r"
            f: fn
                \ ->
                    g
                end
            end
        ";
        let g = r"
            g: fn
                \ ->
                    g
                end
            end
        ";

        for [a, b] in [[f, g], [g, f]] {
            let (syntax_tree, dependencies) = resolve_dependencies(&format!(
                "
                    {a}
                    {b}
                ",
            ));

            let [f, g] = ["f", "g"].map(|name| {
                syntax_tree.function_by_name(name).unwrap().location()
            });

            assert_eq!(
                dependencies_by_function(&dependencies, &syntax_tree),
                [[g], [f]]
            );
        }
    }

    #[test]
    fn function_dependencies_in_the_presence_of_mutual_recursion() {
        let functions = [
            (
                "f",
                vec![
                    r"
                        \ ->
                            0 g
                        end
                    ",
                ],
            ),
            (
                "g",
                vec![
                    r"
                        \ 0 ->
                            0 h
                        end
                    ",
                    r"
                        \ n ->
                            0
                        end
                    ",
                ],
            ),
            (
                "h",
                vec![
                    r"
                        \ 0 ->
                            1 h
                        end
                    ",
                    r"
                        \ n ->
                            n g
                        end
                    ",
                ],
            ),
        ];

        for functions in all_permutations(functions) {
            let [a, b, c] = functions.as_slice() else {
                unreachable!();
            };

            let (syntax_tree, dependencies) = resolve_dependencies(&format!(
                "
                    {a}
                    {b}
                    {c}
                ",
            ));

            let [f, g, h] = ["f", "g", "h"]
                .map(|name| syntax_tree.function_by_name(name).unwrap())
                .map(|function| function.location());

            let clusters =
                dependencies_by_function(&dependencies, &syntax_tree);
            let [cluster_a, cluster_b] = clusters.as_slice() else {
                panic!("Expected two clusters.");
            };

            // `g` and `h` are mutually recursive, so their order is not
            // defined.
            assert!(cluster_a.contains(&g));
            assert!(cluster_a.contains(&h));

            assert_eq!(cluster_b, &vec![f]);
        }
    }

    #[test]
    fn branch_dependencies_in_the_presence_of_mutual_recursion() {
        let functions = [
            r"
                f: fn
                    # a
                    \ 0 ->
                        0 g
                    end

                    # b
                    \ n ->
                        0
                    end
                end
            ",
            r"
                g: fn
                    # c
                    \ 0 ->
                        1 g
                    end

                    # d
                    \ n ->
                        n f
                    end
                end
            ",
        ];

        let [a, b] = functions;

        let (syntax_tree, dependencies) = resolve_dependencies(&format!(
            r"
                {a}
                {b}
            ",
        ));

        let (a, b, c, d) = ["f", "g"]
            .map(|name| syntax_tree.function_by_name(name).unwrap())
            .into_iter()
            .flat_map(|function| {
                function
                    .into_located_function()
                    .branches()
                    .map(|branch| branch.location)
            })
            .collect_tuple()
            .unwrap();

        let mut branches = dependencies
            .clusters()
            .next()
            .unwrap()
            .branches(&syntax_tree)
            .map(|branch| branch.location);
        let (first, second) =
            branches.by_ref().take(2).collect_tuple().unwrap();

        assert_eq!(first, b);
        assert_eq!(second, d);

        // The other two are mutually dependent, so there is no clear order.
        let rest = branches.collect::<BTreeSet<_>>();
        assert!(rest.contains(&a));
        assert!(rest.contains(&c));
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
                end

                g: fn
                    \ ->
                        nop
                    end
                end

                h: fn
                    \ ->
                        # And for some variety, call a function that is placed
                        # before.
                        g
                    end
                end
            ",
        );

        let [f, g, h] = ["f", "g", "h"]
            .map(|name| syntax_tree.function_by_name(name).unwrap().location());

        assert_eq!(
            dependencies_by_function(&dependencies, &syntax_tree),
            [[g], [h], [f]]
        );
    }

    #[test]
    fn consider_local_functions_in_call_graph() {
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
                    end
                end

                g: fn
                    \ ->
                        nop
                    end
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
                    end
                end
            ",
        );

        let [(f, f_a), (h, h_a)] = ["f", "h"].map(|name| {
            let named = syntax_tree
                .function_by_name(name)
                .unwrap()
                .into_located_function();
            let local = named
                .find_single_branch()
                .unwrap()
                .expressions()
                .next()
                .unwrap()
                .into_local_function()
                .map(|function| function.location)
                .unwrap();

            (named.location, local)
        });
        let g = syntax_tree.function_by_name("g").unwrap().location();

        assert_eq!(
            dependencies_by_function(&dependencies, &syntax_tree),
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

    /// # Given a set of functions, iterate over all permutations
    ///
    /// The functions are represented as tuples of a name and a list of
    /// branches.
    ///
    /// Creates all permutations of each provided function (i.e. all possible
    /// orders of branches), computes the cartesian product of all those
    /// functions (i.e. creates all possible combinations of them), and finally,
    /// creates all permutations for each set of functions (again, all possible
    /// orders of those functions).
    ///
    /// Or in other words, it's possible to create each function with any order
    /// of their branches, and to define those functions in any order. Given
    /// those possibilities, this function creates all potential combinations.
    fn all_permutations<'r>(
        functions: impl IntoIterator<Item = (&'r str, Vec<&'r str>)>,
    ) -> impl Iterator<Item = Vec<String>> {
        let permutations_based_on_branch_ordering = functions
            .into_iter()
            .map(|(name, branches)| {
                let mut permutations = Vec::new();

                for permutation in branches.iter().permutations(branches.len())
                {
                    let mut function = format!("{name}: fn\n");

                    for branch in permutation {
                        writeln!(function, "{branch}").unwrap();
                    }

                    writeln!(function, "end").unwrap();

                    permutations.push(function);
                }

                permutations
            })
            .multi_cartesian_product()
            .collect::<Vec<_>>();

        let k = permutations_based_on_branch_ordering.len();
        permutations_based_on_branch_ordering
            .into_iter()
            .permutations(k)
            .flatten()
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn dependencies_by_function(
        dependencies: &Dependencies,
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
