use capi_compiler::{repr::fragments, source_map::SourceMap2};
use capi_process::Process;

use super::Fragment;

#[derive(Clone, Eq, PartialEq)]
pub struct Function {
    pub name: String,
    pub fragments: Vec<Fragment>,
}

impl Function {
    pub fn new(
        function: fragments::Function,
        source_map_2: &SourceMap2,
        process: &Process,
    ) -> Self {
        let fragments = function
            .fragments
            .map(|fragment| {
                Fragment::new(
                    fragment.location,
                    fragment.payload,
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
