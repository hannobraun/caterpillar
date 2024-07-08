use std::collections::BTreeMap;

use capi_process::Location as RuntimeLocation;

use crate::repr::fragments::FragmentId;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct SourceMap {
    instruction_to_fragment: BTreeMap<RuntimeLocation, FragmentId>,
    fragment_to_instruction: BTreeMap<FragmentId, RuntimeLocation>,
}

impl SourceMap {
    pub fn define_mapping(
        &mut self,
        runtime: RuntimeLocation,
        fragment: FragmentId,
    ) {
        self.instruction_to_fragment
            .insert(runtime.clone(), fragment);
        self.fragment_to_instruction.insert(fragment, runtime);
    }

    pub fn instruction_to_fragment(
        &self,
        runtime: &RuntimeLocation,
    ) -> FragmentId {
        self.instruction_to_fragment
            .get(runtime)
            .cloned()
            .expect("Expect every runtime location to map to a syntax location")
    }

    /// Get the runtime location that a given syntax location is mapped to
    ///
    /// Can return `None`, as comments have no mapping to runtime locations.
    pub fn fragment_to_instruction(
        &self,
        fragment: &FragmentId,
    ) -> Option<RuntimeLocation> {
        self.fragment_to_instruction.get(fragment).cloned()
    }
}
