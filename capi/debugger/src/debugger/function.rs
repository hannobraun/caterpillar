use capi_compiler::{
    fragments::{self, Fragments},
    source_map::SourceMap,
};
use capi_process::Process;

use super::Branch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    pub name: Option<String>,
    pub branches: Vec<Branch>,
}

impl Function {
    pub fn new(
        function: fragments::Function,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process,
    ) -> Self {
        let name = function.name;
        let branches = function
            .branches
            .into_iter()
            .map(|branch| Branch::new(branch, fragments, source_map, process))
            .collect();

        Self { name, branches }
    }
}
