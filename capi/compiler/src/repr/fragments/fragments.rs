use std::collections::BTreeMap;

use super::{Fragment, FragmentId};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    pub inner: BTreeMap<FragmentId, Fragment>,
    pub by_function: Vec<Function>,
}

impl Fragments {
    pub fn find_function(&self, fragment_id: &FragmentId) -> Option<&Function> {
        for function in &self.by_function {
            for fragment in function.fragments.clone() {
                if &fragment.id() == fragment_id {
                    return Some(function);
                }
            }
        }

        None
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
