use std::collections::HashMap;

use super::FragmentId;

#[derive(Debug)]
pub struct Replacements {
    pub(super) inner: HashMap<FragmentId, FragmentId>,
}

impl Replacements {
    pub fn insert(&mut self, old: FragmentId, new: FragmentId) {
        self.inner.insert(old, new);
    }
}
