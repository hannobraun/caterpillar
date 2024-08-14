use crate::syntax::Branch;

pub fn find_clusters(branches: Vec<Branch>) -> Vec<Function> {
    let mut clusters = Vec::<Function>::new();

    for branch in branches {
        let cluster = clusters
            .iter_mut()
            .find(|cluster| cluster.name == branch.name);

        match cluster {
            Some(cluster) => {
                cluster.branches.push(branch);
            }
            None => {
                clusters.push(Function {
                    name: branch.name.clone(),
                    branches: vec![branch],
                });
            }
        }
    }

    clusters
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub branches: Vec<Branch>,
}
