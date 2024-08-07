use crate::syntax::Function;

pub fn find_clusters(functions: Vec<Function>) -> Vec<Cluster> {
    let mut clusters = Vec::<Cluster>::new();

    for function in functions {
        let cluster = clusters
            .iter_mut()
            .find(|cluster| cluster.name == function.name);

        match cluster {
            Some(cluster) => {
                cluster.members.push(function);
            }
            None => {
                clusters.push(Cluster {
                    name: function.name.clone(),
                    members: vec![function],
                });
            }
        }
    }

    clusters
}

#[derive(Clone)]
pub struct Cluster {
    pub name: String,
    pub members: Vec<Function>,
}
