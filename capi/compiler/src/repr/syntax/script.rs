use super::{Function, SyntaxBuilder};

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Script {
    pub functions: Vec<Function>,
}

impl Script {
    pub fn function<'r>(
        &mut self,
        name: &str,
        arguments: impl IntoIterator<Item = &'r str>,
        f: impl FnOnce(&mut SyntaxBuilder),
    ) -> &mut Self {
        let body = {
            let mut expressions = Vec::new();
            f(&mut SyntaxBuilder::new(&mut expressions));
            expressions
        };

        self.functions.push(Function {
            name: name.to_string(),
            args: arguments.into_iter().map(String::from).collect(),
            body,
        });

        self
    }
}
