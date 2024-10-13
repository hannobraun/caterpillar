use crate::syntax::Cluster;

/// # The program's named functions, organized as a call graph
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CallGraph {
    pub clusters: Vec<Cluster>,
}
