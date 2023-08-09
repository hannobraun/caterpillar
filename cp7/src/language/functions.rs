use std::collections::BTreeMap;

use super::{
    intrinsics,
    runtime::data_stack::{DataStack, DataStackResult},
    syntax::SyntaxHandle,
    value,
};

#[derive(Debug)]
pub struct Functions {
    inner: BTreeMap<String, Function>,
}

impl Functions {
    pub fn new() -> Self {
        let mut self_ = Self {
            inner: BTreeMap::new(),
        };

        let intrinsics = [
            ("+", intrinsics::add as Intrinsic),
            ("clone", intrinsics::clone),
            ("delay_ms", intrinsics::delay_ms),
            ("print_line", intrinsics::print_line),
            ("fn", intrinsics::fn_),
        ];

        for (name, intrinsic) in intrinsics {
            self_.register_intrinsic(name, intrinsic)
        }

        self_
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
