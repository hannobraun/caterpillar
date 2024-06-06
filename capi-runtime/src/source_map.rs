use std::collections::BTreeMap;

use crate::{runtime, syntax};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct SourceMap {
    runtime_to_syntax: BTreeMap<runtime::InstructionAddress, syntax::Location>,
    syntax_to_runtime: BTreeMap<syntax::Location, runtime::InstructionAddress>,
}

impl SourceMap {
    pub fn define_mapping(
        &mut self,
        runtime: runtime::InstructionAddress,
        location: syntax::Location,
    ) {
        self.runtime_to_syntax.insert(runtime, location.clone());
        self.syntax_to_runtime.insert(location, runtime);
    }

    /// Get `LineLocation` for the provided `InstructionAddress`
    ///
    /// This might return `None`, as not all instructions have locations in the
    /// code. Return instructions are an example of that.
    ///
    /// This shouldn't matter, since users can't set breakpoints there, nor do
    /// those instructions produce errors, nor should they show up in a call
    /// stack. So in cases where you actually need a location, this should
    /// return one.
    pub fn address_to_location(
        &self,
        address: &runtime::InstructionAddress,
    ) -> Option<syntax::Location> {
        self.runtime_to_syntax.get(address).cloned()
    }

    /// Get `InstructionAddress` for the provided `LineLocation`
    ///
    /// Returns an `Option`, because this might be called for a mapping that has
    /// not been defined. If that happens, it is likely to be a bug outside of
    /// this module.
    pub fn location_to_address(
        &self,
        location: &syntax::Location,
    ) -> Option<runtime::InstructionAddress> {
        self.syntax_to_runtime.get(location).cloned()
    }
}
