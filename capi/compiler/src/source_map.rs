use std::collections::BTreeMap;

use capi_process::InstructionAddr;

use crate::repr::fragments::FragmentId;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct SourceMap {
    instruction_to_fragment: BTreeMap<InstructionAddr, FragmentId>,
    fragment_to_instruction: BTreeMap<FragmentId, InstructionAddr>,
}

impl SourceMap {
    pub fn define_mapping(
        &mut self,
        instruction: InstructionAddr,
        fragment: FragmentId,
    ) {
        self.instruction_to_fragment.insert(instruction, fragment);
        self.fragment_to_instruction.insert(fragment, instruction);
    }

    pub fn instruction_to_fragment(
        &self,
        instruction: &InstructionAddr,
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
    ) -> Option<InstructionAddr> {
        self.fragment_to_instruction.get(fragment).cloned()
    }
}
