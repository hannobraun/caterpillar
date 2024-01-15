use std::collections::BTreeMap;

use crate::{
    repr::eval::fragments::FragmentId,
    value::{self, Value},
};

use super::functions::Functions;

#[derive(Debug)]
pub struct UserDefinedItems<'r> {
    pub bindings: &'r mut BTreeMap<String, Value>,
    pub functions: &'r mut Functions,
    pub tests: &'r mut Functions,
}

impl UserDefinedItems<'_> {
    pub fn define_binding(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub fn define_function(&mut self, name: FunctionName, body: value::Block) {
        let function = UserDefinedFunction {
            name: name.clone(),
            body,
        };
        self.functions.0.insert(name.value, function);
    }

    pub fn define_test(&mut self, name: FunctionName, body: value::Block) {
        let function = UserDefinedFunction {
            name: name.clone(),
            body,
        };
        self.tests.0.insert(name.value, function);
    }

    pub fn functions(&self) -> impl Iterator<Item = &UserDefinedFunction> {
        self.functions.0.values()
    }

    pub fn tests(&self) -> impl Iterator<Item = &UserDefinedFunction> {
        self.tests.0.values()
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
