use std::collections::BTreeMap;

use crate::{runtime::InstructionAddress, syntax};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct SourceMap {
    address_to_location: BTreeMap<InstructionAddress, syntax::Location>,
    location_to_address: BTreeMap<syntax::Location, InstructionAddress>,
}

impl SourceMap {
    pub fn define_mapping(
        &mut self,
        address: InstructionAddress,
        location: syntax::Location,
    ) {
        self.address_to_location.insert(address, location.clone());
        self.location_to_address.insert(location, address);
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
        address: &InstructionAddress,
    ) -> Option<syntax::Location> {
        self.address_to_location.get(address).cloned()
    }

    /// Get `InstructionAddress` for the provided `LineLocation`
    ///
    /// Returns an `Option`, because this might be called for a mapping that has
    /// not been defined. If that happens, it is likely to be a bug outside of
    /// this module.
    pub fn location_to_address(
        &self,
        location: &syntax::Location,
    ) -> Option<InstructionAddress> {
        self.location_to_address.get(location).cloned()
    }
}
