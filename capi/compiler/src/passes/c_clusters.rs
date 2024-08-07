use crate::syntax::Function;

pub fn find_clusters(functions: Vec<Function>) -> Vec<Cluster> {
    let mut clusters = Vec::new();

    for function in functions {
        clusters.push(Cluster {
            members: vec![function.clone()],
        });
    }

    clusters
}

pub struct Cluster {
    pub members: Vec<Function>,
}
