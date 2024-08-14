use capi_compiler::{
    fragments::{self, Fragments},
    source_map::SourceMap,
};
use capi_process::Process;

use super::Expression;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Branch {
    pub name: String,
    pub body: Vec<Expression>,
}

impl Branch {
    pub fn new(
        function: fragments::Function,
        branch: fragments::Branch,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process,
    ) -> Self {
        let name = function.name.expect(
            "At this point, we should only be dealing with named functions. \
            If there are any anonymous functions on the call stack, we should \
            be seeing the named function in which it was defined here.",
        );
        let body = fragments
            .inner
            .iter_from(branch.start)
            .cloned()
            .filter_map(|fragment| {
                Expression::new(fragment, fragments, source_map, process)
            })
            .collect();

        Self { name, body }
    }
}
