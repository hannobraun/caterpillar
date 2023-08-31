use std::collections::BTreeMap;

use crate::language::repr::{eval::value, fragments::FragmentId};

use super::data_stack::{DataStack, DataStackResult};

#[derive(Debug)]
pub struct Functions {
    inner: BTreeMap<String, Function>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn register_intrinsic(&mut self, name: &str, intrinsic: Intrinsic) {
        self.inner
            .insert(name.into(), Function::Intrinsic(intrinsic));
    }

    pub fn define(&mut self, name: value::Symbol, body: value::Block) {
        let function = Function::UserDefined(UserDefined { body });
        self.inner.insert(name.0, function);
    }

    pub fn resolve(&self, name: &str) -> Result<&Function, ResolveError> {
        self.inner
            .get(name)
            .ok_or(ResolveError { name: name.into() })
    }

    pub fn replace(&mut self, old: FragmentId, new: FragmentId) {
        for function in self.inner.values_mut() {
            if let Function::UserDefined(UserDefined { body }) = function {
                if let Some(fragment_id) = &mut body.start {
                    if *fragment_id == old {
                        *fragment_id = new;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Function {
    Intrinsic(Intrinsic),
    UserDefined(UserDefined),
}

pub type Intrinsic = fn(&mut Functions, &mut DataStack) -> DataStackResult<()>;

#[derive(Debug)]
pub struct UserDefined {
    pub body: value::Block,
}

#[derive(Debug, thiserror::Error)]
#[error("Error resolving function `{name}`")]
pub struct ResolveError {
    pub name: String,
}
