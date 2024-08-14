use crate::syntax::{Branch, Function};

pub fn find_functions(branches: Vec<Branch>) -> Vec<Function> {
    let mut functions = Vec::<Function>::new();

    for branch in branches {
        let cluster = functions
            .iter_mut()
            .find(|cluster| cluster.name == branch.name);

        match cluster {
            Some(cluster) => {
                cluster.branches.push(branch);
            }
            None => {
                functions.push(Function {
                    name: branch.name.clone(),
                    branches: vec![branch],
                });
            }
        }
    }

    functions
}
