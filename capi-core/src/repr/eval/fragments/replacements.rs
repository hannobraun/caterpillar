use std::collections::HashMap;

use super::FragmentId;

#[derive(Debug)]
pub struct Replacements {
    old_to_new: HashMap<FragmentId, FragmentId>,
    new_to_old: HashMap<FragmentId, FragmentId>,
}

impl Replacements {
    pub fn new() -> Self {
        Self {
            old_to_new: HashMap::new(),
            new_to_old: HashMap::new(),
        }
    }

    pub fn insert(&mut self, old: FragmentId, new: FragmentId) {
        self.old_to_new.insert(old, new);
        self.new_to_old.insert(new, old);
    }

    pub fn replaced_by(&self, new: FragmentId) -> Option<FragmentId> {
        self.new_to_old.get(&new).copied()
    }

    pub fn take(&mut self) -> Vec<Replacement> {
        self.new_to_old.clear();
        self.old_to_new
            .drain()
            .map(|(old, new)| Replacement { old, new })
            .collect()
    }
}

pub struct Replacement {
    pub old: FragmentId,
    pub new: FragmentId,
}
