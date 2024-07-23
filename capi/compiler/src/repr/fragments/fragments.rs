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
            for fragment in self.inner.iter_from(function.start) {
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
    pub fn iter_from(&self, id: FragmentId) -> impl Iterator<Item = &Fragment> {
        let mut next = Some(id);

        iter::from_fn(move || {
            let id = next.take()?;
            let fragment = self.inner.get(&id)?;

            next = fragment.next();

            Some(fragment)
        })
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub start: FragmentId,
}
