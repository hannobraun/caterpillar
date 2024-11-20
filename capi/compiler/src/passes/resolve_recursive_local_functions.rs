use crate::code::{Expression, FunctionLocation, Functions, OrderedFunctions};

pub fn resolve_recursive_local_functions(
    functions: &mut Functions,
    ordered_functions: &OrderedFunctions,
) {
    for cluster in ordered_functions.clusters_from_leaves() {
        for location in cluster.functions.values() {
            let function = functions
                .by_location_mut(location)
                .expect("Function referred to from cluster must exist.");

            let branches = function.destructure();

            for branch in branches {
                let (body, _) = branch.destructure();

                for expression in body {
                    if let Expression::UnresolvedLocalFunction { .. } =
                        expression.fragment
                    {
                        let location = FunctionLocation::AnonymousFunction {
                            location: expression.location,
                        };

                        if let Some((&index, _)) = cluster
                            .functions
                            .iter()
                            .find(|(_, l)| **l == location)
                        {
                            *expression.fragment =
                                Expression::LocalFunctionRecursive { index };
                        }
                    }
                }
            }
        }
    }
}
