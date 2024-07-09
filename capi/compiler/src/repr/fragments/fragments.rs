use std::{collections::BTreeMap, iter};

use super::{Fragment, FragmentId};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    pub inner: FragmentMap,
    pub by_function: Vec<Function>,
}

impl Fragments {
    pub fn find_function(&self, fragment_id: &FragmentId) -> Option<&Function> {
        for function in &self.by_function {
            for fragment in function.fragments.clone().drain() {
                if &fragment.id() == fragment_id {
                    return Some(function);
                }
            }
        }

        None
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FragmentMap {
    pub inner: BTreeMap<FragmentId, Fragment>,
}

impl FragmentMap {
    pub fn remove(&mut self, id: &FragmentId) -> Option<Fragment> {
        self.inner.remove(id)
    }
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
    inner: FragmentMap,
}

impl FunctionFragments {
    pub fn new(first: Option<FragmentId>, inner: FragmentMap) -> Self {
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

    pub fn drain(&mut self) -> impl Iterator<Item = Fragment> + '_ {
        iter::from_fn(move || self.remove_first())
    }
}
