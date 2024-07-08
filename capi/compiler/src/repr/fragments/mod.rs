mod address;
mod fragment;
mod id;
mod payload;

pub use self::{
    address::FragmentAddress, fragment::Fragment, id::FragmentId,
    payload::FragmentPayload,
};

use std::collections::{BTreeMap, BTreeSet};

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
