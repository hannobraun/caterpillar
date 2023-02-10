use std::collections::BTreeMap;

use super::{Args, Function};

pub struct Registry {
    pub inner: BTreeMap<(String, Args), Function>,
}

impl Registry {
    pub fn new() -> Self {
        let inner = BTreeMap::new();
        Self { inner }
    }
}
