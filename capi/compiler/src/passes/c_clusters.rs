use std::collections::BTreeMap;

use crate::syntax::Function;

pub fn find_clusters(functions: Vec<Function>) -> Vec<Cluster> {
    let mut clusters = Vec::new();
    let mut groups = BTreeMap::new();

    for mut function in functions {
        let next_group_index = groups.entry(function.name.clone()).or_default();
        function.group_index = Some(*next_group_index);
        *next_group_index += 1;

        clusters.push(Cluster {
            members: vec![function.clone()],
        });
    }

    clusters
}

pub struct Cluster {
    pub members: Vec<Function>,
}
