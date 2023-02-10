use super::{Args, Function};

pub struct Registry {
    inner: Vec<Function>,
}

impl Registry {
    pub fn new() -> Self {
        let inner = Vec::new();
        Self { inner }
    }

    pub fn define(
        &mut self,
        name: impl Into<String>,
        args: impl Into<Args>,
        body: &str,
    ) {
        self.inner.push(Function::new(name, args, body));
    }

    #[cfg(test)]
    pub fn resolve(
        &self,
        name: impl Into<String>,
        _: impl IntoIterator<Item = crate::cp::Value>,
    ) -> Option<&Function> {
        let name = name.into();

        let mut by_name = self
            .inner
            .iter()
            .filter(|f| f.name == name)
            .collect::<Vec<_>>();
        by_name.sort_by_key(|f| f.args.inner.len());
        by_name.reverse();

        by_name.pop()
    }

    pub fn get(
        &self,
        name: impl Into<String>,
        args: impl Into<Args>,
    ) -> Option<&Function> {
        let name = name.into();
        let args = args.into();

        self.inner
            .iter()
            .find(|function| function.name == name && function.args == args)
    }

    pub fn get_mut(
        &mut self,
        name: impl Into<String>,
        args: impl Into<Args>,
    ) -> Option<&mut Function> {
        let name = name.into();
        let args = args.into();

        self.inner
            .iter_mut()
            .find(|function| function.name == name && function.args == args)
    }
}

#[cfg(test)]
mod tests {
    use crate::cp::{functions::Args, Arg, Type, Value};

    use super::Registry;

    #[test]
    fn resolve_simplest_signature() {
        let mut registry = Registry::new();
        registry.define("name", [Arg::Type(Type::Bool)], "");
        registry.define("name", [], "");

        let f = registry.resolve("name", [Value::Bool(true)]).unwrap();

        assert_eq!(f.name, "name");
        assert_eq!(f.args, Args::from([]));
    }
}
