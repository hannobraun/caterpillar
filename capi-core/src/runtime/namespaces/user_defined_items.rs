use std::collections::BTreeMap;

use crate::{
    module::function::{FunctionName, UserDefinedFunction},
    value::{self, Value},
};

use super::functions::Functions;

#[derive(Debug, Default)]
pub struct UserDefinedItems {
    pub bindings: BTreeMap<String, Value>,
    pub functions: Functions,

    // It's unnecessary and somewhat misleading that we store tests by name,
    // same as functions. While they *have* a named, they can't be *called by*
    // their name.
    //
    // This was a deliberate design decision:
    //
    // - They are meant to be called by the test harness, and calling them
    //   directly does not seem useful.
    // - If a case is found where that is useful, a regular function can be
    //   defined, and the test can be made a wrapper around that function.
    // - If they were regular parts of the namespace (which once was the case),
    //   it would be too easy to accidentally define tests that conflict with
    //   the functions they test (as experience has shown).
    // - And in any case, if they were theoretically callable, they wouldn't be
    //   in practice, as test names are strings and can contain whitespace,
    //   which words can't express (*words* in the language sense, not the
    //   general sense).
    //
    // The only reason we do that here, is to share the replacement code with
    // tests and functions. I suspect that this code will be removed in the
    // future, as my pipeline evaluation refactor progresses.
    //
    // Once that is the case, we should probably just change this to be a
    // `Vec<Function>`, or something along those lines.
    pub tests: Functions,
}

impl UserDefinedItems {
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
