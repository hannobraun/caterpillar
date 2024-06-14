use crate::{process::Process, syntax};

use super::Expression;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub expressions: Vec<Expression>,
}

impl Function {
    pub fn new(function: syntax::Function, process: &Process) -> Self {
        Self {
            name: function.name,
            expressions: function
                .syntax
                .into_iter()
                .map(|expression| Expression::new(expression, process))
                .collect::<Vec<_>>(),
        }
    }
}
