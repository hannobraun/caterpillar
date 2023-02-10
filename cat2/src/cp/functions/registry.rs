use std::collections::BTreeMap;

use super::{Args, Function};

pub struct Registry {
    pub inner: BTreeMap<(String, Args), Function>,
}
