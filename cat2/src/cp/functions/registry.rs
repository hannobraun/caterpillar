use super::{Args, Function};

pub struct Registry {
    inner: Vec<(Args, Function)>,
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
        let args = args.into();
        self.inner
            .push((args.clone(), Function::new(name, args, body)));
    }

    #[cfg(test)]
    pub fn resolve(
        &self,
        name: impl Into<String>,
        _: impl Into<Args>,
    ) -> Option<&Function> {
        let name = name.into();
        self.inner
            .iter()
            .find(|(_, f)| f.name == name)
            .map(|(_, f)| f)
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
            .find(|(a, function)| function.name == name && a == &args)
            .map(|(_, function)| function)
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
            .find(|(a, function)| function.name == name && a == &args)
            .map(|(_, function)| function)
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
