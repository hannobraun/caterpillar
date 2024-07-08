use std::collections::BTreeSet;

use super::{Expression, SyntaxBuilder};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Functions {
    pub names: BTreeSet<String>,
    pub inner: Vec<Function>,
}

impl Functions {
    pub fn define<'r>(
        &mut self,
        name: &str,
        args: impl IntoIterator<Item = &'r str>,
        f: impl FnOnce(&mut SyntaxBuilder),
    ) {
        if self.names.contains(name) {
            panic!("Can't re-define existing function `{name}`.");
        }

        let mut expressions = Vec::new();
        f(&mut SyntaxBuilder::new(&mut expressions));

        self.names.insert(name.to_string());
        self.inner.push(Function {
            name: name.to_string(),
            args: args.into_iter().map(String::from).collect(),
            expressions,
        });
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub expressions: Vec<Expression>,
}
