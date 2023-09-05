use std::collections::HashMap;

use super::FragmentId;

#[derive(Debug)]
pub struct Replacements {
    old_to_new: HashMap<FragmentId, FragmentId>,
}

impl Replacements {
    pub fn new() -> Self {
        Self {
            old_to_new: HashMap::new(),
        }
    }

    pub fn insert(&mut self, old: FragmentId, new: FragmentId) {
        self.old_to_new.insert(old, new);
    }

    pub fn take(&mut self) -> Vec<(FragmentId, FragmentId)> {
        self.old_to_new.drain().collect()
    }
}
