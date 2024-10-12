use crate::syntax::Cluster;

use super::{search::FoundFunction, FunctionLocation, NamedFunctions};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    /// # The named functions in the root context
    pub named_functions: NamedFunctions,

    /// # The function clusters
    pub clusters: Vec<Cluster>,
}

impl Fragments {
    /// # Find the cluster containing a given function
    ///
    /// ## Panics
    ///
    /// Panics, if the provided location does not refer to a named function.
    pub fn find_cluster_by_named_function(
        &self,
        location: &FunctionLocation,
    ) -> Option<&Cluster> {
        let FunctionLocation::NamedFunction { index } = location else {
            panic!("Can't search for cluster by anonymous function.");
        };

        self.clusters
            .iter()
            .find(|cluster| cluster.functions.values().any(|i| i == index))
    }

    /// # Find the function with the provided name
    ///
    /// ## Implementation Note
    ///
    /// There is currently a function with a similar name on `FragmentMap`. Once
    /// the ongoing refactoring on fragment addressing has finished, that
    /// function can be removed, and this one can take over its name.
    pub fn find_function_by_name(&self, name: &str) -> Option<FoundFunction> {
        self.named_functions
            .inner
            .iter()
            .find_map(|(&index, function)| {
                if function.name.as_deref() == Some(name) {
                    let location = FunctionLocation::NamedFunction { index };
                    Some(FoundFunction {
                        function: function.clone(),
                        location,
                    })
                } else {
                    None
                }
            })
    }
}
