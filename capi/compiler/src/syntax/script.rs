use std::collections::BTreeSet;

use capi_runtime::Value;

use super::{function::Pattern, Branch, Expression, Function};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Script {
    pub functions: Vec<Function>,
}

impl Script {
    pub fn function(
        &mut self,
        name: &str,
        branches: impl FnOnce(&mut BranchBuilder) -> &mut BranchBuilder,
    ) -> &mut Self {
        let branches = {
            let mut builder = BranchBuilder {
                branches: Vec::new(),
            };
            branches(&mut builder);
            builder.branches
        };

        self.functions.push(Function {
            name: Some(name.to_string()),
            branches,
            environment: BTreeSet::new(),
        });

        self
    }
}

pub struct BranchBuilder {
    branches: Vec<Branch>,
}

impl BranchBuilder {
    pub fn branch(
        &mut self,
        parameters: impl FnOnce(&mut PatternBuilder) -> &mut PatternBuilder,
        body: impl FnOnce(&mut ExpressionBuilder),
    ) -> &mut Self {
        let parameters = {
            let mut builder = PatternBuilder {
                patterns: Vec::new(),
            };
            parameters(&mut builder);
            builder.patterns
        };
        let body = {
            let mut builder = ExpressionBuilder {
                expressions: Vec::new(),
            };
            body(&mut builder);
            builder.expressions
        };

        self.branches.push(Branch { parameters, body });

        self
    }
}

pub struct PatternBuilder {
    patterns: Vec<Pattern>,
}

impl PatternBuilder {
    pub fn ident(&mut self, name: &str) -> &mut Self {
        self.patterns
            .push(Pattern::Identifier { name: name.into() });
        self
    }

    pub fn lit(&mut self, value: impl Into<Value>) -> &mut Self {
        self.patterns.push(Pattern::Literal {
            value: value.into(),
        });
        self
    }
}

#[derive(Debug)]
pub struct ExpressionBuilder {
    expressions: Vec<Expression>,
}

impl ExpressionBuilder {
    pub fn fun(
        &mut self,
        branches: impl FnOnce(&mut BranchBuilder) -> &mut BranchBuilder,
    ) -> &mut Self {
        let branches = {
            let mut builder = BranchBuilder {
                branches: Vec::new(),
            };
            branches(&mut builder);
            builder.branches
        };

        self.push_expression(Expression::Function {
            function: Function {
                name: None,
                branches,
                environment: BTreeSet::new(),
            },
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
