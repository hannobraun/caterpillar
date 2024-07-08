mod address;
mod payload;

pub use self::{address::FragmentAddress, payload::FragmentPayload};

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

use super::syntax::Location;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    pub functions: BTreeSet<String>,
    pub by_function: Vec<Function>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub fragments: FunctionFragments,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FunctionFragments {
    first: Option<FragmentId>,
    inner: FunctionFragmentsInner,
}

impl FunctionFragments {
    pub fn new(
        first: Option<FragmentId>,
        inner: FunctionFragmentsInner,
    ) -> Self {
        Self { first, inner }
    }
}

impl FunctionFragments {
    pub fn remove_first(&mut self) -> Option<Fragment> {
        let first = self.first.take()?;
        let first = self
            .inner
            .remove(&first)
            .expect("`self.first` must be present in `self.inner`");

        self.first = first.address.next;

        Some(first)
    }
}

impl Iterator for FunctionFragments {
    type Item = Fragment;

    fn next(&mut self) -> Option<Self::Item> {
        self.remove_first()
    }
}

type FunctionFragmentsInner = BTreeMap<FragmentId, Fragment>;

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct FragmentId {
    pub hash: blake3::Hash,
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragment {
    pub address: FragmentAddress,
    pub payload: FragmentPayload,
    pub location: Location,
}

impl Fragment {
    pub fn id(&self) -> FragmentId {
        let mut hasher = blake3::Hasher::new();

        self.address.hash(&mut hasher);
        self.payload.hash(&mut hasher);

        FragmentId {
            hash: hasher.finalize(),
        }
    }
}
