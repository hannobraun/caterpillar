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
        _: &SourceMap2,
        process: &Process,
    ) -> Self {
        let fragments = function
            .fragments
            .map(|fragment| FragmentModel::new(fragment, source_map, process))
            .collect();

        Self {
            name: function.name,
            fragments,
        }
    }
}
