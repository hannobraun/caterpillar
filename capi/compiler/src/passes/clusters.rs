use crate::syntax::Branch;

pub fn find_clusters(functions: Vec<Branch>) -> Vec<Cluster> {
    let mut clusters = Vec::<Cluster>::new();

    for function in functions {
        let cluster = clusters
            .iter_mut()
            .find(|cluster| cluster.name == function.name);

        match cluster {
            Some(cluster) => {
                cluster.branches.push(function);
            }
            None => {
                clusters.push(Cluster {
                    name: function.name.clone(),
                    branches: vec![function],
                });
            }
        }
    }

    clusters
}

#[derive(Clone)]
pub struct Cluster {
    pub name: String,
    pub branches: Vec<Branch>,
}
