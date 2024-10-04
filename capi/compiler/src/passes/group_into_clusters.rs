use std::{collections::BTreeMap, iter};

use crate::{
    fragments::FunctionIndexInCluster,
    syntax::{Cluster, Clusters, Function, NamedFunctionIndex},
};

pub fn group_into_clusters(functions: Vec<Function>) -> Clusters {
    let mut functions = iter::successors(Some(0), |i| Some(i + 1))
        .map(NamedFunctionIndex)
        .zip(functions)
        .collect::<BTreeMap<_, _>>();

    // This is just a placeholder implementation, while support for clusters is
    // still being implemented.
    let clusters = functions
        .iter_mut()
        .map(|(named_function_index, function)| {
            let function_index_in_cluster = FunctionIndexInCluster(0);

            function.index_in_cluster = Some(function_index_in_cluster);

            Cluster {
                functions: BTreeMap::from([(
                    function_index_in_cluster,
                    *named_function_index,
                )]),
            }
        })
        .collect();

    Clusters {
        functions,
        clusters,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::{
        fragments::FunctionIndexInCluster,
        host::NoHost,
        passes::{parse, resolve_identifiers, tokenize},
        syntax::{Cluster, Clusters, NamedFunctionIndex},
    };

    #[test]
    fn no_recursion() {
        let clusters = group_into_clusters(
            r"
                main: {
                    \ ->
                        f
                }

                f: {
                    \ ->
                        nop
                }
            ",
        );

        assert_eq!(
            clusters.clusters.into_iter().collect::<BTreeSet<_>>(),
            [
                (FunctionIndexInCluster(0), NamedFunctionIndex(0)),
                (FunctionIndexInCluster(0), NamedFunctionIndex(1)),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: BTreeMap::from([indices]),
            })
            .collect::<BTreeSet<_>>(),
        );
    }

    #[test]
    fn self_recursion() {
        let clusters = group_into_clusters(
            r"
                main: {
                    \ ->
                        f
                }

                f: {
                    \ ->
                        f
                }
            ",
        );

        assert_eq!(
            clusters.clusters.into_iter().collect::<BTreeSet<_>>(),
            [
                (FunctionIndexInCluster(0), NamedFunctionIndex(0)),
                (FunctionIndexInCluster(0), NamedFunctionIndex(1))
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: BTreeMap::from([indices])
            })
            .collect::<BTreeSet<_>>(),
        );
    }

    #[test]
    #[should_panic] // known limitation
    fn mutual_recursion() {
        let clusters = group_into_clusters(
            r"
                main: {
                    \ ->
                        f
                }

                f: {
                    \ ->
                        g
                }

                g: {
                    \ ->
                        f
                }
            ",
        );

        assert_eq!(
            clusters.clusters.into_iter().collect::<BTreeSet<_>>(),
            [
                [(FunctionIndexInCluster(0), NamedFunctionIndex(0))].as_slice(),
                [
                    (FunctionIndexInCluster(0), NamedFunctionIndex(1)),
                    (FunctionIndexInCluster(1), NamedFunctionIndex(2))
                ]
                .as_slice(),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: indices.iter().copied().collect(),
            })
            .collect::<BTreeSet<_>>(),
        );
    }

    fn group_into_clusters(source: &str) -> Clusters {
        let tokens = tokenize(source);
        let mut functions = parse(tokens);
        resolve_identifiers::<NoHost>(&mut functions);
        super::group_into_clusters(functions)
    }
}
