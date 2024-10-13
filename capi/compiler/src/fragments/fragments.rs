use super::{CallGraph, NamedFunctions};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    /// # The named functions in the root context
    pub named_functions: NamedFunctions,

    /// # The function clusters
    pub call_graph: CallGraph,
}
