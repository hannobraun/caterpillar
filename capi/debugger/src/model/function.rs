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
        source_map: &SourceMap2,
        process: &Process,
    ) -> Self {
        let fragments = function
            .fragments
            .map(|expression| {
                Fragment::new(
                    expression.location,
                    expression.payload,
                    source_map,
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
