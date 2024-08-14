use std::collections::BTreeSet;

use capi_process::Value;

use super::{function::Pattern, Branch, Expression};

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Script {
    pub branches: Vec<Branch>,
}

impl Script {
    pub fn function(
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

        self.branches.extend(branches);

        self
    }
}

pub struct BranchBuilder {
    branches: Vec<Branch>,
}

impl BranchBuilder {
    pub fn branch(
        &mut self,
        name: &str,
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

        self.branches.push(Branch {
            name: name.to_string(),
            parameters,
            body,
        });

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
    pub fn block(
        &mut self,
        body: impl FnOnce(&mut ExpressionBuilder),
    ) -> &mut Self {
        let body = {
            let mut builder = ExpressionBuilder {
                expressions: Vec::new(),
            };
            body(&mut builder);
            builder.expressions
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
