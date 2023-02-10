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
        let name = name.into();
        self.inner.push((args.into(), Function::new(name, body)));
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
