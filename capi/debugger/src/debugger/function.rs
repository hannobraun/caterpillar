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
        code: &Code,
        process: &Process,
    ) -> Self {
        let name = function.name.clone();
        let branches = function
            .branches
            .into_iter()
            .map(|branch| {
                Branch::new(
                    name.clone().expect(
                        "At this point, we should only be dealing with named \
                        functions. If there are any anonymous functions on the \
                        call stack, we should be seeing the named function in \
                        which it was defined here.",
                    ),
                    branch,
                    &code.fragments,
                    &code.source_map,
                    process,
                )
            })
            .collect();

        Self { name, branches }
    }
}
