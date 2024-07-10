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
            if let Some(start) = function.start {
                for fragment in self.inner.clone().drain_from(start) {
                    if &fragment.id() == fragment_id {
                        return Some(function);
                    }
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

    pub fn drain_from(
        &mut self,
        id: FragmentId,
    ) -> impl Iterator<Item = Fragment> + '_ {
        let mut next = Some(id);

        iter::from_fn(move || {
            let id = next.take()?;
            let fragment = self.remove(&id)?;

            next = fragment.address.next;

            Some(fragment)
        })
    }

    pub fn iter_from(&self, id: FragmentId) -> impl Iterator<Item = &Fragment> {
        let mut next = Some(id);

        iter::from_fn(move || {
            let id = next.take()?;
            let fragment = self.inner.get(&id)?;

            next = fragment.address.next;

            Some(fragment)
        })
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub start: Option<FragmentId>,
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
    pub fn drain(&mut self) -> Box<dyn Iterator<Item = Fragment> + '_> {
        self.first
            .map(|id| {
                Box::new(self.inner.drain_from(id))
                    as Box<dyn Iterator<Item = Fragment>>
            })
            .unwrap_or_else(|| Box::new(iter::empty()))
    }
}
