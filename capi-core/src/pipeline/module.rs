use std::collections::BTreeMap;

use crate::repr::eval::value::{self, Value};

#[derive(Debug, Default)]
pub struct Module {
    // Bindings do not actually belong here.
    //
    // Right now it doesn't make a difference, because there is just one global
    // scope that the single global module is mixed up in. But eventually,
    // bindings will be a feature of a local scope, not of any module. Hence,
    // having them here is a mistake.
    //
    // In addition, bindings are the only reason that the core builtins access
    // `context.global_module`. Once those no longer live here, that field can
    // be moved to the platform context of the compile-time platform.
    pub bindings: BTreeMap<String, Value>,

    pub functions: BTreeMap<String, Function>,

    /// The tests defined in this module
    ///
    /// Tests are not stored by name, as they can not be called by it. This is
    /// a deliberate design decision:
    ///
    /// - They are meant to be called by the test harness, and calling them
    ///   directly does not seem useful.
    /// - If a case is found where that is useful, a regular function can be
    ///   defined, and the test can be made a wrapper around that function.
    /// - If they were regular parts of the namespace (which once was the case),
    ///   it would be too easy to accidentally define tests that conflict with
    ///   the functions they test (as experience has shown).
    /// - And in any case, if they were theoretically callable, they wouldn't be
    ///   in practice, as test names are strings and can contain whitespace,
    ///   which words can't express (*words* in the language sense, not the
    ///   general sense).
    pub tests: Vec<Function>,
}

impl Module {
    pub fn merge(&mut self, other: &mut Self) {
        self.bindings.append(&mut other.bindings);
        self.functions.append(&mut other.functions);
        self.tests.append(&mut other.tests);
    }

    pub fn define_binding(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    pub fn define_function(&mut self, name: String, body: value::Block) {
        let function = Function {
            name: name.clone(),

            body,
        };
        self.functions.insert(name, function);
    }

    pub fn define_test(&mut self, name: String, body: value::Block) {
        let function = Function { name, body };
        self.tests.push(function);
    }

    pub fn functions(&self) -> impl Iterator<Item = &Function> {
        self.functions.values()
    }

    pub fn tests(&self) -> impl Iterator<Item = &Function> {
        self.tests.iter()
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub body: value::Block,
}
