use std::{cmp::Ordering, fmt};

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct FragmentId {
    hash: blake3::Hash,
}

impl FragmentId {
    pub(super) fn new(hash: blake3::Hash) -> Self {
        Self { hash }
    }

    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        hasher.update(self.hash.as_bytes());
    }
}

impl Ord for FragmentId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash.as_bytes().cmp(other.hash.as_bytes())
    }
}

impl PartialOrd for FragmentId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for FragmentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}
