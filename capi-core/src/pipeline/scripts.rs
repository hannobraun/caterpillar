use std::collections::BTreeMap;

use crate::value;

#[derive(Default)]
pub struct Scripts {
    pub inner: BTreeMap<Vec<value::Symbol>, String>,
}
