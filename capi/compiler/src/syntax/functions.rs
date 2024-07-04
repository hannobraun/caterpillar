use std::collections::BTreeSet;

use super::{Expression, Location, SyntaxBuilder};

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

        let mut syntax = Vec::new();
        f(&mut SyntaxBuilder::new(
            Location::first_in_function(name.to_string()),
            &mut syntax,
        ));

        self.names.insert(name.to_string());
        self.inner.push(Function {
            name: name.to_string(),
            args: args.into_iter().map(String::from).collect(),
            syntax,
        });
    }

    pub fn get_from_location(&self, location: Location) -> Option<&Function> {
        self.inner
            .iter()
            .find(|function| function.name == location.function())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub syntax: Vec<Expression>,
}
