use capi_compiler::fragments;
use capi_process::Process;
use capi_protocol::updates::Code;

use super::Branch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    pub name: Option<String>,
    pub branches: Vec<Branch>,
}

impl Function {
    pub fn new(
        function: fragments::Function,
        branch: fragments::Branch,
        code: &Code,
        process: &Process,
    ) -> Self {
        let name = function.name.clone();
        let branches = vec![Branch::new(
            function,
            branch,
            &code.fragments,
            &code.source_map,
            process,
        )];
        Self { name, branches }
    }
}
