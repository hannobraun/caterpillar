use super::{Function, Functions, SyntaxBuilder};

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Script {
    pub functions: Functions,
}

impl Script {
    pub fn function<'r>(
        &mut self,
        name: &str,
        args: impl IntoIterator<Item = &'r str>,
        f: impl FnOnce(&mut SyntaxBuilder),
    ) {
        let mut expressions = Vec::new();
        f(&mut SyntaxBuilder::new(&mut expressions));

        self.functions.inner.push(Function {
            name: name.to_string(),
            args: args.into_iter().map(String::from).collect(),
            expressions,
        });
    }
}
