use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Functions {
    pub inner: BTreeMap<&'static str, usize>,
}
