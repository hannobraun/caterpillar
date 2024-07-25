use capi_compiler::{
    repr::fragments::{self, Fragments},
    source_map::SourceMap,
};
use capi_process::Process;
use capi_protocol::host::GameEngineHost;

use super::Expression;

#[derive(Clone, Eq, PartialEq)]
pub struct Function {
    pub name: String,
    pub body: Vec<Expression>,
}

impl Function {
    pub fn new(
        function: fragments::Function,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process<GameEngineHost>,
    ) -> Self {
        let mut body = Vec::new();

        body.extend(
            fragments
                .inner
                .iter_from(function.start)
                .cloned()
                .filter_map(|fragment| {
                    Expression::new(fragment, source_map, process)
                }),
        );

        Self {
            name: function.name,
            body,
        }
    }
}
