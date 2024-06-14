use std::collections::BTreeMap;

use crate::{runtime, syntax};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SourceMap {
    runtime_to_syntax: BTreeMap<runtime::Location, syntax::Location>,
    syntax_to_runtime: BTreeMap<syntax::Location, runtime::Location>,
}

impl SourceMap {
    pub fn define_mapping(
        &mut self,
        runtime: runtime::Location,
        syntax: syntax::Location,
    ) {
        self.runtime_to_syntax
            .insert(runtime.clone(), syntax.clone());
        self.syntax_to_runtime.insert(syntax, runtime);
    }

    pub fn runtime_to_syntax(
        &self,
        runtime: &runtime::Location,
    ) -> syntax::Location {
        self.runtime_to_syntax
            .get(runtime)
            .cloned()
            .expect("Expect every runtime location to map to a syntax location")
    }

    /// Get the runtime location that a given syntax location is mapped to
    ///
    /// Can return `None`, as comments have no mapping to runtime locations.
    pub fn syntax_to_runtime(
        &self,
        syntax: &syntax::Location,
    ) -> Option<runtime::Location> {
        self.syntax_to_runtime.get(syntax).cloned()
    }
}
