use std::collections::BTreeMap;

use crate::{InstructionAddress, LineLocation};

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct SourceMap {
    address_to_location: BTreeMap<InstructionAddress, LineLocation>,
}

impl SourceMap {
    pub fn define_mapping(
        &mut self,
        address: InstructionAddress,
        location: LineLocation,
    ) {
        self.address_to_location.insert(address, location);
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
    ) -> Option<LineLocation> {
        self.address_to_location.get(&address).cloned()
    }
}
