use std::collections::BTreeMap;

use crate::{InstructionAddress, SourceLocation};

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct SourceMap {
    address_to_location: BTreeMap<InstructionAddress, SourceLocation>,
    location_to_address: BTreeMap<SourceLocation, InstructionAddress>,
}

impl SourceMap {
    pub fn define_mapping(
        &mut self,
        address: InstructionAddress,
        location: SourceLocation,
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
        address: InstructionAddress,
    ) -> Option<SourceLocation> {
        self.address_to_location.get(&address).cloned()
    }
}
