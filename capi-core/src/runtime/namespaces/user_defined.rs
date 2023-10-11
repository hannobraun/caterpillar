use std::collections::BTreeMap;

use crate::{repr::eval::fragments::FragmentId, value};

pub struct UserDefined<'r> {
    pub inner: &'r mut BTreeMap<String, UserDefinedFunction>,
}

impl UserDefined<'_> {
    pub fn define(
        &mut self,
        name: FunctionName,
        body: value::Block,
        is_test: bool,
    ) {
        let function = UserDefinedFunction {
            name: name.clone(),
            body,
            is_test,
        };
        self.inner.insert(name.value, function);
    }

    pub fn iter(&self) -> impl Iterator<Item = &UserDefinedFunction> {
        self.inner.values()
    }
}

#[derive(Clone, Debug)]
pub struct UserDefinedFunction {
    pub name: FunctionName,
    pub body: value::Block,
    pub is_test: bool,
}

#[derive(Clone, Debug)]
pub struct FunctionName {
    pub value: String,
    pub fragment: Option<FragmentId>,
}
