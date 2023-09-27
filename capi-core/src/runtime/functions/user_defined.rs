use std::collections::BTreeMap;

use crate::{repr::eval::fragments::FragmentId, value};

pub struct UserDefinedFunctions<'r> {
    pub(super) inner: &'r mut BTreeMap<String, UserDefinedFunction>,
}

impl UserDefinedFunctions<'_> {
    pub fn define(&mut self, name: FunctionName, body: value::Block) {
        let function = UserDefinedFunction {
            name: name.clone(),
            body,
        };
        self.inner.insert(name.value, function);
    }
}

#[derive(Clone, Debug)]
pub struct UserDefinedFunction {
    pub name: FunctionName,
    pub body: value::Block,
}

#[derive(Clone, Debug)]
pub struct FunctionName {
    pub value: String,
    pub fragment: Option<FragmentId>,
}
