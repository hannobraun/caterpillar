use std::collections::{BTreeSet, VecDeque};

use crate::code::{
    Branch, BranchLocation, Cluster, Expression, Functions, OrderedFunctions,
};

pub fn sort_non_divergent_branches(
    functions: &Functions,
    ordered_functions: &mut OrderedFunctions,
) {
    for cluster in ordered_functions.clusters_from_leaves_mut() {
        let Some(divergent_functions) = &cluster.divergent_functions else {
            panic!(
                "The compiler pass that find divergent functions must run \
                before this one."
            );
        };

        let mut non_divergent_branches_unordered = cluster
            .functions(functions)
            .filter(|function| {
                !divergent_functions.contains(&function.location)
            })
            .flat_map(|function| function.branches())
            .collect::<VecDeque<_>>();

        let mut non_divergent_branches: Vec<BranchLocation> = Vec::new();
        let mut already_pushed = BTreeSet::new();

        while let Some(branch) = non_divergent_branches_unordered.pop_front() {
            if branch_refers_to_functions_not_yet_in_list(
                &branch,
                cluster,
                &non_divergent_branches,
                functions,
            ) {
                if already_pushed.contains(&branch.location) {
                    let mut list_of_functions = String::new();

                    for function in cluster.functions(functions) {
                        let entry = &format!(
                            "- {}\n",
                            function.location.display(functions)
                        );
                        list_of_functions.push_str(entry);
                    }

                    unreachable!(
                        "Sorting non-divergent branches ran into endless loop \
                        while processing the following cluster:\n\
                        {list_of_functions}"
                    );
                }

                already_pushed.insert(branch.location.clone());
                non_divergent_branches_unordered.push_back(branch);
                continue;
            } else {
                non_divergent_branches.push(branch.location);
                already_pushed.clear();
            }
        }

        cluster.non_divergent_branches = Some(non_divergent_branches);
    }
}

fn branch_refers_to_functions_not_yet_in_list(
    branch: &Branch,
    cluster: &Cluster,
    non_divergent_branches: &[BranchLocation],
    functions: &Functions,
) -> bool {
    for fragment in branch.body.values() {
        match fragment {
            Expression::CallToUserDefinedFunctionRecursive {
                index, ..
            } => {
                let location = cluster.functions.get(index).expect(
                    "Target of resolved function call must exist in cluster.",
                );
                let function = functions
                    .by_location(location)
                    .expect("Function referred to from cluster must exist.");

                if function.branches().all(|branch| {
                    !non_divergent_branches.contains(&branch.location)
                }) {
                    return true;
                }
            }
            Expression::LocalFunctionRecursive { index } => {
                let location = cluster.functions.get(index).expect(
                    "Target of resolved recursive call must exist in cluster.",
                );
                let function = functions
                    .by_location(location)
                    .expect("Function referred to from cluster must exist.");

                if function.branches.values().all(|branch| {
                    branch_refers_to_functions_not_yet_in_list(
                        branch,
                        cluster,
                        non_divergent_branches,
                        functions,
                    )
                }) {
                    return true;
                }
            }
            _ => {}
        }
    }

    false
}
