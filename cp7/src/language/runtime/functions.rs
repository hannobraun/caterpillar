use std::collections::BTreeMap;

use crate::language::repr::eval::{fragments::FragmentId, value};

use super::{data_stack::DataStackResult, evaluator::Evaluator};

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

    pub fn define(&mut self, name: FunctionName, body: value::Block) {
        let function = Function::UserDefined(UserDefined {
            name: name.clone(),
            body,
        });
        self.inner.insert(name.value, function);
    }

    pub fn resolve(&self, name: &str) -> Result<&Function, ResolveError> {
        self.inner
            .get(name)
            .ok_or(ResolveError { name: name.into() })
    }

    pub fn replace(&mut self, old: FragmentId, new: FragmentId) {
        for function in self.inner.values_mut() {
            if let Function::UserDefined(UserDefined { body, .. }) = function {
                if body.start == old {
                    body.start = new;
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

pub type Intrinsic = fn(&mut Evaluator) -> DataStackResult<()>;

#[derive(Debug)]
pub struct UserDefined {
    pub name: FunctionName,
    pub body: value::Block,
}

#[derive(Clone, Debug)]
pub struct FunctionName {
    pub value: String,
    pub fragment: Option<FragmentId>,
}

#[derive(Debug, thiserror::Error)]
#[error("Error resolving function `{name}`")]
pub struct ResolveError {
    pub name: String,
}
