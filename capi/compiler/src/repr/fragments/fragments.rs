use std::{collections::BTreeMap, iter};

use super::{Fragment, FragmentId, FragmentParent};

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    pub inner: FragmentMap,
    pub by_function: Vec<Function>,
}

impl Fragments {
    pub fn find_function_by_name(&self, name: &str) -> Option<&Function> {
        self.by_function
            .iter()
            .find(|function| function.name == name)
    }

    pub fn find_function_by_fragment(
        &self,
        fragment_id: &FragmentId,
    ) -> Option<&Function> {
        let mut fragment_id = *fragment_id;

        loop {
            let fragment = self.inner.inner.get(&fragment_id)?;
            match &fragment.parent {
                FragmentParent::Fragment { id } => {
                    fragment_id = *id;
                }
                FragmentParent::Function { name } => {
                    return self.find_function_by_name(name);
                }
            };
        }
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
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
