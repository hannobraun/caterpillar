mod repr;
mod resolve;

pub use self::repr::{Dependencies, DependencyCluster};

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use itertools::Itertools;

    use crate::{
        code::{
            syntax::{Function, FunctionLocation, NamedFunction, SyntaxTree},
            Dependencies, FunctionCalls, Tokens,
        },
        host::NoHost,
    };

    // This test suite uses a custom compiler pipeline that creates all possible
    // permutations of the code, to make sure these tests don't accidentally
    // rely on a specific order of definitions in the code.
    //
    // As a consequence, these tests must not query compiler output by position,
    // as other test suites are generally free to do.

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
        let compiler_output = resolve_dependencies(
            "
                f: fn
                    br ->
                        g
                    end
                end
                g: fn
                    br ->
                        g
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
    fn function_dependencies_in_the_presence_of_mutual_recursion() {
        let compiler_output = resolve_dependencies(
            "
                f: fn
                    br ->
                        0 g
                    end
                end

                g: fn
                    br 0 ->
                        0 h
                    end
                    br n ->
                        0
                    end
                end

                h: fn
                    br 0 ->
                        1 h
                    end
                    br n ->
                        n g
                    end
                end
            ",
        );

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

    #[test]
    fn branch_dependencies_in_the_presence_of_mutual_recursion() {
        let compiler_output = resolve_dependencies(
            "
                f: fn
                    # a
                    br 0 ->
                        0 g
                    end

                    # b
                    br n ->
                        0
                    end
                end

                g: fn
                    # c
                    br 0 ->
                        1 g
                    end

                    # d
                    br n ->
                        n f
                    end
                end
            ",
        );

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

                ["a", "b", "c", "d"].map(|name| branches.remove(name).unwrap())
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

    #[test]
    fn sort_clusters_by_call_graph() {
        let compiler_output = resolve_dependencies(
            "
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
            "
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
        let permutations =
            original.named_functions.values().cloned().permutations(k);

        let mut syntax_trees = Vec::new();

        for named_functions in permutations {
            permutate_rest_of_named_functions(
                SyntaxTree::default(),
                named_functions.into_iter(),
                &mut syntax_trees,
            );
        }

        syntax_trees.into_iter()
    }

    fn permutate_rest_of_named_functions(
        syntax_tree: SyntaxTree,
        mut named_functions: impl Iterator<Item = NamedFunction> + Clone,
        syntax_trees: &mut Vec<SyntaxTree>,
    ) {
        match named_functions.next() {
            Some(named_function) => {
                for function in permutate_function(named_function.inner) {
                    let named_function = NamedFunction {
                        comment: named_function.comment.clone(),
                        name: named_function.name.clone(),
                        inner: function,
                    };
                    let mut syntax_tree = syntax_tree.clone();

                    syntax_tree.named_functions.push(named_function);
                    permutate_rest_of_named_functions(
                        syntax_tree,
                        named_functions.clone(),
                        syntax_trees,
                    );
                }
            }
            None => {
                syntax_trees.push(syntax_tree);
            }
        }
    }

    fn permutate_function(
        function: Function,
    ) -> impl Iterator<Item = Function> {
        let mut functions = Vec::new();

        let k = function.branches.len();
        let permutations = function.branches.into_values().permutations(k);

        for branches in permutations {
            functions.push(Function {
                branches: branches.into_iter().collect(),
            });
        }

        functions.into_iter()
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
