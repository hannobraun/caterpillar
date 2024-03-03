use std::collections::BTreeMap;

use crate::repr::eval::value::{self, Value};

use super::{functions::Functions, Function, FunctionName};

#[derive(Debug, Default)]
pub struct Module {
    pub bindings: BTreeMap<String, Value>,
    pub functions: Functions,

    // It's unnecessary and somewhat misleading that we store tests by name,
    // same as functions. While they *have* a name, they can't be *called by*
    // it.
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

impl Module {
    pub fn merge(&mut self, other: &mut Self) {
        self.bindings.append(&mut other.bindings);
        self.functions.0.append(&mut other.functions.0);
        self.tests.0.append(&mut other.tests.0);
    }

    pub fn define_binding(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub fn define_function(&mut self, name: String, body: value::Block) {
        let function = Function {
            name: FunctionName {
                value: name.clone(),
            },
            body,
        };
        self.functions.0.insert(name, function);
    }

    pub fn define_test(&mut self, name: String, body: value::Block) {
        let function = Function {
            name: FunctionName {
                value: name.clone(),
            },
            body,
        };
        self.tests.0.insert(name, function);
    }

    pub fn functions(&self) -> impl Iterator<Item = &Function> {
        self.functions.0.values()
    }

    pub fn tests(&self) -> impl Iterator<Item = &Function> {
        self.tests.0.values()
    }
}
