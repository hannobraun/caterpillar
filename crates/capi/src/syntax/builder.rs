use capi_process::runtime::Value;

use crate::syntax::{Expression, ExpressionKind, Location};

#[derive(Debug)]
pub struct SyntaxBuilder<'r> {
    next_location: Location,
    expressions: &'r mut Vec<Expression>,
}

impl<'r> SyntaxBuilder<'r> {
    pub fn new(function: String, expressions: &'r mut Vec<Expression>) -> Self {
        Self {
            next_location: Location::first_in_function(function),
            expressions,
        }
    }

    pub fn b(
        &mut self,
        names: impl IntoIterator<Item = impl Into<String>>,
    ) -> &mut Self {
        self.push_expression(ExpressionKind::Binding {
            names: names.into_iter().map(Into::into).collect(),
        })
    }

    pub fn c(&mut self, text: &str) -> &mut Self {
        self.push_expression(ExpressionKind::Comment { text: text.into() })
    }

    pub fn v(&mut self, value: impl Into<Value>) -> &mut Self {
        self.push_expression(ExpressionKind::Value(value.into()))
    }

    pub fn w(&mut self, name: &str) -> &mut Self {
        self.push_expression(ExpressionKind::Word { name: name.into() })
    }

    fn push_expression(&mut self, kind: ExpressionKind) -> &mut Self {
        let location = self.next_location.increment();
        self.expressions.push(Expression::new(kind, location));
        self
    }
}
