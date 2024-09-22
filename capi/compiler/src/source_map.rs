use std::collections::BTreeMap;

use capi_runtime::InstructionAddress;

use crate::fragments::{Fragment, FragmentId, Hash};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct SourceMap {
    instruction_to_fragment: BTreeMap<InstructionAddress, FragmentId>,
    fragment_to_instructions: BTreeMap<Hash<Fragment>, Vec<InstructionAddress>>,
}

impl SourceMap {
    pub fn define_mapping(
        &mut self,
        instruction: InstructionAddress,
        fragment: FragmentId,
    ) {
        self.instruction_to_fragment.insert(instruction, fragment);
        self.fragment_to_instructions
            .entry(fragment.this)
            .or_default()
            .push(instruction);
    }

    /// Get the ID of the fragment that the given instruction maps to
    ///
    /// Can return `None`, as there are a few compiler-generated instructions
    /// that call the `main` function.
    pub fn instruction_to_fragment(
        &self,
        instruction: &InstructionAddress,
    ) -> Option<Hash<Fragment>> {
        self.instruction_to_fragment
            .get(instruction)
            .cloned()
            .map(|id| id.this)
    }

    /// Get the address of the instruction that the given fragment maps to
    ///
    /// Can return a reference to an empty `Vec`, as comments have no mapping to
    /// instructions.
    pub fn fragment_to_instructions(
        &self,
        fragment: &FragmentId,
    ) -> &Vec<InstructionAddress> {
        static EMPTY: Vec<InstructionAddress> = Vec::new();

        self.fragment_to_instructions
            .get(&fragment.this)
            .unwrap_or(&EMPTY)
    }
}
