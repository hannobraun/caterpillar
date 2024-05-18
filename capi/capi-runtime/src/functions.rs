use std::collections::BTreeSet;

use crate::{syntax::Expression, SourceLocation};

use super::syntax::Syntax;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Functions {
    pub names: BTreeSet<String>,
    pub inner: Vec<Function>,
}

impl Functions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: &str, f: impl FnOnce(&mut Syntax)) {
        if self.names.contains(name) {
            panic!("Can't re-define existing function `{name}`.");
        }

        let mut syntax = Vec::new();
        f(&mut Syntax::new(name.to_string(), &mut syntax));

        self.names.insert(name.to_string());
        self.inner.push(Function {
            name: name.to_string(),
            syntax,
        });
    }

    pub fn get_from_location(
        &self,
        location: SourceLocation,
    ) -> Option<&Function> {
        self.inner
            .iter()
            .find(|function| function.name == location.function())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub syntax: Vec<Expression>,
}
