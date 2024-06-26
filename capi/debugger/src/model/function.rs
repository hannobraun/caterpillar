use capi_compiler::{source_map::SourceMap, syntax};
use capi_process::Process;

use super::Expression;

#[derive(Clone, Eq, PartialEq)]
pub struct Function {
    pub name: String,
    pub expressions: Vec<Expression>,
}

impl Function {
    pub fn new(
        function: syntax::Function,
        source_map: &SourceMap,
        process: &Process,
    ) -> Self {
        Self {
            name: function.name,
            expressions: function
                .syntax
                .into_iter()
                .map(|expression| {
                    Expression::new(expression, source_map, process)
                })
                .collect::<Vec<_>>(),
        }
    }
}
