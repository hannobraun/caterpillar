use crate::{
    breakpoints,
    process::{self, Process},
    source_map::SourceMap,
    syntax,
};

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
        breakpoints: &breakpoints::State,
        _: &process::State,
        process: &Process,
    ) -> Self {
        Self {
            name: function.name,
            expressions: function
                .syntax
                .into_iter()
                .map(|expression| {
                    Expression::new(
                        expression,
                        source_map,
                        breakpoints,
                        process,
                    )
                })
                .collect::<Vec<_>>(),
        }
    }
}
