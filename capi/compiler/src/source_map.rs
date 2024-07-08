use std::collections::BTreeMap;

use capi_process::Location as RuntimeLocation;

use crate::repr::{fragments::FragmentId, syntax};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct SourceMap {
    runtime_to_syntax: BTreeMap<RuntimeLocation, FragmentId>,
    syntax_to_runtime: BTreeMap<FragmentId, RuntimeLocation>,
}

impl SourceMap {
    pub fn define_mapping(
        &mut self,
        runtime: RuntimeLocation,
        syntax: FragmentId,
    ) {
        self.runtime_to_syntax.insert(runtime.clone(), syntax);
        self.syntax_to_runtime.insert(syntax, runtime);
    }

    pub fn runtime_to_syntax(&self, runtime: &RuntimeLocation) -> FragmentId {
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
        syntax: &FragmentId,
    ) -> Option<RuntimeLocation> {
        self.syntax_to_runtime.get(syntax).cloned()
    }
}

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct SourceMap2 {
    runtime_to_syntax: BTreeMap<RuntimeLocation, syntax::Location>,
    syntax_to_runtime: BTreeMap<syntax::Location, RuntimeLocation>,
}

impl SourceMap2 {
    pub fn define_mapping(
        &mut self,
        runtime: RuntimeLocation,
        syntax: syntax::Location,
    ) {
        self.runtime_to_syntax
            .insert(runtime.clone(), syntax.clone());
        self.syntax_to_runtime.insert(syntax, runtime);
    }

    pub fn runtime_to_syntax(
        &self,
        runtime: &RuntimeLocation,
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
    ) -> Option<RuntimeLocation> {
        self.syntax_to_runtime.get(syntax).cloned()
    }
}
