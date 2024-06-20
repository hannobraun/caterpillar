use crate::{breakpoints, process::Process, source_map::SourceMap, syntax};

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
        _: &breakpoints::State,
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
