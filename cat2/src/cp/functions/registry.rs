use super::{Args, Function};

pub struct Registry {
    inner: Vec<(String, Args, Function)>,
}

impl Registry {
    pub fn new() -> Self {
        let inner = Vec::new();
        Self { inner }
    }

    pub fn insert(
        &mut self,
        name: impl Into<String>,
        args: impl Into<Args>,
        body: &str,
    ) {
        self.inner
            .push((name.into(), args.into(), Function::new(body)));
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
            .find(|(n, a, _)| n == &name && a == &args)
            .map(|(_, _, function)| function)
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
            .find(|(n, a, _)| n == &name && a == &args)
            .map(|(_, _, function)| function)
    }
}
