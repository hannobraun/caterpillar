use std::collections::BTreeSet;

use capi_process::Value;

use super::{Expression, Function};

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Script {
    pub functions: Vec<Function>,
}

impl Script {
    pub fn function<'r>(
        &mut self,
        name: &str,
        arguments: impl IntoIterator<Item = &'r str>,
        body: impl FnOnce(&mut ExpressionBuilder),
    ) -> &mut Self {
        let arguments = arguments.into_iter().map(String::from).collect();
        let body = {
            let mut expressions = Vec::new();
            body(&mut ExpressionBuilder {
                expressions: &mut expressions,
            });
            expressions
        };

        self.functions.push(Function {
            name: name.to_string(),
            arguments,
            body,
        });

        self
    }
}

#[derive(Debug)]
pub struct ExpressionBuilder<'r> {
    expressions: &'r mut Vec<Expression>,
}

impl ExpressionBuilder<'_> {
    pub fn block(
        &mut self,
        f: impl FnOnce(&mut ExpressionBuilder),
    ) -> &mut Self {
        let body = {
            let mut body = Vec::new();
            f(&mut ExpressionBuilder {
                expressions: &mut body,
            });
            body
        };

        self.push_expression(Expression::Block {
            body,
            environment: BTreeSet::new(),
        })
    }

    pub fn bind(
        &mut self,
        names: impl IntoIterator<Item = impl Into<String>>,
    ) -> &mut Self {
        self.push_expression(Expression::Binding {
            names: names.into_iter().map(Into::into).collect(),
        })
    }

    pub fn c(&mut self, text: &str) -> &mut Self {
        self.push_expression(Expression::Comment { text: text.into() })
    }

    pub fn ident(&mut self, name: &str) -> &mut Self {
        self.push_expression(Expression::Identifier {
            name: name.into(),
            target: None,
            is_known_to_be_in_tail_position: false,
        })
    }

    pub fn v(&mut self, value: impl Into<Value>) -> &mut Self {
        self.push_expression(Expression::Value(value.into()))
    }

    fn push_expression(&mut self, expression: Expression) -> &mut Self {
        self.expressions.push(expression);
        self
    }
}
