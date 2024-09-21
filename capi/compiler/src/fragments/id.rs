use std::cmp::Ordering;

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct FragmentId {
    pub(super) hash: [u8; 32],
}

impl FragmentId {
    pub(super) fn new(hash: blake3::Hash) -> Self {
        Self { hash: hash.into() }
    }
}

impl Ord for FragmentId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash.cmp(&other.hash)
    }
}

impl PartialOrd for FragmentId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
