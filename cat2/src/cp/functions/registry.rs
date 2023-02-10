use std::collections::BTreeMap;

use crate::cp::tokenize;

use super::{Args, Function};

pub struct Registry {
    pub inner: BTreeMap<(String, Args), Function>,
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
        self.inner.insert(
            (name.into(), args.into()),
            Function {
                tokens: tokenize(body),
            },
        );
    }
}
