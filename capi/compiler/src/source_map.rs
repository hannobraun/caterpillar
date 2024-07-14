use std::collections::BTreeMap;

use capi_process::{InstructionIndex, Location as RuntimeLocation};

use crate::repr::fragments::FragmentId;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct SourceMap {
    instruction_to_fragment: BTreeMap<InstructionIndex, FragmentId>,
    fragment_to_instruction: BTreeMap<FragmentId, InstructionIndex>,
}

impl SourceMap {
    pub fn define_mapping(
        &mut self,
        runtime: RuntimeLocation,
        fragment: FragmentId,
    ) {
        self.instruction_to_fragment.insert(runtime.index, fragment);
        self.fragment_to_instruction.insert(fragment, runtime.index);
    }

    pub fn instruction_to_fragment(
        &self,
        instruction: &InstructionIndex,
    ) -> FragmentId {
        self.instruction_to_fragment
            .get(instruction)
            .cloned()
            .expect("Expect every runtime location to map to a syntax location")
    }

    /// Get the runtime location that a given syntax location is mapped to
    ///
    /// Can return `None`, as comments have no mapping to runtime locations.
    pub fn fragment_to_instruction(
        &self,
        fragment: &FragmentId,
    ) -> Option<InstructionIndex> {
        self.fragment_to_instruction.get(fragment).cloned()
    }
}
