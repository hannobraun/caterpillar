use std::collections::BTreeMap;

use super::{Args, Function};

pub struct Registry {
    inner: BTreeMap<(String, Args), Function>,
}

impl Registry {
    pub fn new() -> Self {
        let inner = BTreeMap::new();
        Self { inner }
    }

    pub fn insert(
        &mut self,
        name: impl Into<String>,
        args: impl Into<Args>,
        body: &str,
    ) {
        self.inner
            .insert((name.into(), args.into()), Function::new(body));
    }

    pub fn get(
        &self,
        name: impl Into<String>,
        args: impl Into<Args>,
    ) -> Option<&Function> {
        self.inner.get(&(name.into(), args.into()))
    }

    pub fn get_mut(
        &mut self,
        name: impl Into<String>,
        args: impl Into<Args>,
    ) -> Option<&mut Function> {
        self.inner.get_mut(&(name.into(), args.into()))
    }
}
