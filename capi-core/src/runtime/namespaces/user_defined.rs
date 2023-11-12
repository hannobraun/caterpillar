use std::collections::BTreeMap;

use crate::{
    repr::eval::fragments::FragmentId,
    value::{self, Value},
};

pub struct UserDefined<'r> {
    pub bindings: &'r mut BTreeMap<String, Value>,
    pub functions: &'r mut BTreeMap<String, UserDefinedFunction>,
}

impl UserDefined<'_> {
    pub fn define_binding(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub fn define_function(&mut self, name: FunctionName, body: value::Block) {
        let function = UserDefinedFunction {
            name: name.clone(),
            body,
        };
        self.functions.insert(name.value, function);
    }

    pub fn functions(&self) -> impl Iterator<Item = &UserDefinedFunction> {
        self.functions.values()
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
