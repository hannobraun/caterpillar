use std::collections::BTreeMap;

use crate::language::{syntax::SyntaxHandle, value};

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
        let function = Function::UserDefined { body };
        self.inner.insert(name.0, function);
    }

    pub fn resolve(&self, name: &str) -> Result<&Function, ResolveError> {
        self.inner
            .get(name)
            .ok_or(ResolveError { name: name.into() })
    }

    pub fn replace(&mut self, old: SyntaxHandle, new: SyntaxHandle) {
        for function in self.inner.values_mut() {
            if let Function::UserDefined { body } = function {
                if let Some(handle) = &mut body.0 {
                    if handle.hash == old.hash {
                        *handle = new;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Function {
    Intrinsic(Intrinsic),
    UserDefined { body: value::Block },
}

pub type Intrinsic = fn(&mut Functions, &mut DataStack) -> DataStackResult<()>;

#[derive(Debug, thiserror::Error)]
#[error("Error resolving function `{name}`")]
pub struct ResolveError {
    pub name: String,
}
