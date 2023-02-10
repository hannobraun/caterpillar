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
        self.inner.iter().find(|f| f.name == name)
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
    use super::Registry;

    #[test]
    fn resolve() {
        let mut registry = Registry::new();
        registry.define("name", [], "");

        let f = registry.resolve("name", []).unwrap();
        assert_eq!(f.name, "name");
    }
}
