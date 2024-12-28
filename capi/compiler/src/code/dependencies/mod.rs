mod repr;
mod resolve;

pub use self::repr::{Dependencies, DependencyCluster};

#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, BTreeSet},
        fmt::Write,
    };

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
        let compiler_output = resolve_dependencies(
            "
                f: fn
                    br ->
                        g
                    end
                end
                g: fn
                    br ->
                        nop
                    end
                end
            ",
        );

        for (syntax_tree, dependencies) in compiler_output {
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
                br ->
                    g
                end
            end
        ";
        let g = r"
            g: fn
                br ->
                    g
                end
            end
        ";

        for [a, b] in [[f, g], [g, f]] {
            let compiler_output = resolve_dependencies(&format!(
                "
                    {a}
                    {b}
                ",
            ));

            for (syntax_tree, dependencies) in compiler_output {
                let [f, g] = ["f", "g"].map(|name| {
                    syntax_tree.function_by_name(name).unwrap().location()
                });

                assert_eq!(
                    dependencies_by_function(&dependencies, &syntax_tree),
                    [[g], [f]]
                );
            }
        }
    }

    #[test]
    fn function_dependencies_in_the_presence_of_mutual_recursion() {
        let functions = [
            (
                "f",
                vec![
                    r"
                        br ->
                            0 g
                        end
                    ",
                ],
            ),
            (
                "g",
                vec![
                    r"
                        br 0 ->
                            0 h
                        end
                    ",
                    r"
                        br n ->
                            0
                        end
                    ",
                ],
            ),
            (
                "h",
                vec![
                    r"
                        br 0 ->
                            1 h
                        end
                    ",
                    r"
                        br n ->
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

            let compiler_output = resolve_dependencies(&format!(
                "
                    {a}
                    {b}
                    {c}
                ",
            ));

            for (syntax_tree, dependencies) in compiler_output {
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
    }

    #[test]
    fn branch_dependencies_in_the_presence_of_mutual_recursion() {
        let functions = [
            (
                "f",
                vec![
                    r"
                        # a
                        br 0 ->
                            0 g
                        end
                    ",
                    r"
                        # b
                        br n ->
                            0
                        end
                    ",
                ],
            ),
            (
                "g",
                vec![
                    r"
                        # c
                        br 0 ->
                            1 g
                        end
                    ",
                    r"
                        # d
                        br n ->
                            n f
                        end
                    ",
                ],
            ),
        ];

        for functions in all_permutations(functions) {
            let [a, b] = functions.as_slice() else {
                unreachable!();
            };

            let compiler_output = resolve_dependencies(&format!(
                r"
                    {a}
                    {b}
                ",
            ));

            for (syntax_tree, dependencies) in compiler_output {
                let [a, b, c, d] = {
                    let mut branches = ["f", "g"]
                        .map(|name| syntax_tree.function_by_name(name).unwrap())
                        .into_iter()
                        .flat_map(|function| {
                            function.into_located_function().branches().map(
                                |branch| {
                                    let name = branch
                                        .comment
                                        .clone()
                                        .unwrap()
                                        .lines
                                        .into_iter()
                                        .next()
                                        .unwrap();
                                    (name, branch.location)
                                },
                            )
                        })
                        .collect::<BTreeMap<_, _>>();

                    ["a", "b", "c", "d"]
                        .map(|name| branches.remove(name).unwrap())
                };

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
        }
    }

    #[test]
    fn sort_clusters_by_call_graph() {
        let compiler_output = resolve_dependencies(
            r"
                f: fn
                    br ->
                        # Call a function that comes is placed after this one in
                        # the source code.
                        h
                    end
                end

                g: fn
                    br ->
                        nop
                    end
                end

                h: fn
                    br ->
                        # And for some variety, call a function that is placed
                        # before.
                        g
                    end
                end
            ",
        );

        for (syntax_tree, dependencies) in compiler_output {
            let [f, g, h] = ["f", "g", "h"].map(|name| {
                syntax_tree.function_by_name(name).unwrap().location()
            });

            assert_eq!(
                dependencies_by_function(&dependencies, &syntax_tree),
                [[g], [h], [f]]
            );
        }
    }

    #[test]
    fn consider_local_functions_in_call_graph() {
        let compiler_output = resolve_dependencies(
            r"
                f: fn
                    br ->
                        fn
                            br ->
                                # Call a function that is placed after this one
                                # in the source code.
                                h
                            end
                        end
                    end
                end

                g: fn
                    br ->
                        nop
                    end
                end

                h: fn
                    br ->
                        fn
                            br ->
                                # And for some variety, call a function that is
                                # placed before.
                                g
                            end
                        end
                    end
                end
            ",
        );

        for (syntax_tree, dependencies) in compiler_output {
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
    }

    fn resolve_dependencies(
        input: &str,
    ) -> impl Iterator<Item = (SyntaxTree, Dependencies)> {
        let tokens = Tokens::tokenize(input);
        let syntax_tree = SyntaxTree::parse(tokens);

        permutate_syntax_tree(syntax_tree).map(|syntax_tree| {
            let function_calls = FunctionCalls::resolve(&syntax_tree, &NoHost);
            let dependencies =
                Dependencies::resolve(&syntax_tree, &function_calls);

            (syntax_tree, dependencies)
        })
    }

    fn permutate_syntax_tree(
        original: SyntaxTree,
    ) -> impl Iterator<Item = SyntaxTree> {
        let k = original.named_functions.len();
        let permutations = original.named_functions.values().permutations(k);

        let mut syntax_trees = Vec::new();

        // This permutates only the order of named functions. Eventually, I'd
        // like to permutate the order of branches in each function too.
        for permutation in permutations {
            let mut syntax_tree = SyntaxTree::default();

            for named_function in permutation {
                syntax_tree.named_functions.push(named_function.clone());
            }

            syntax_trees.push(syntax_tree);
        }

        syntax_trees.into_iter()
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
