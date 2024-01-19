use std::collections::BTreeMap;

use crate::value;

pub struct Scripts {
    pub inner: BTreeMap<Vec<value::Symbol>, String>,
}
