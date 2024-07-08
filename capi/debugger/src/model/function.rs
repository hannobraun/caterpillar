use capi_compiler::{
    repr::fragments,
    source_map::{SourceMap, SourceMap2},
};
use capi_process::Process;

use super::FragmentModel;

#[derive(Clone, Eq, PartialEq)]
pub struct Function {
    pub name: String,
    pub fragments: Vec<FragmentModel>,
}

impl Function {
    pub fn new(
        function: fragments::Function,
        source_map: &SourceMap,
        source_map_2: &SourceMap2,
        process: &Process,
    ) -> Self {
        let fragments = function
            .fragments
            .map(|fragment| {
                FragmentModel::new(
                    fragment.location,
                    fragment.payload,
                    source_map,
                    source_map_2,
                    process,
                )
            })
            .collect();

        Self {
            name: function.name,
            fragments,
        }
    }
}
