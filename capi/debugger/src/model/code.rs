use anyhow::anyhow;
use capi_compiler::fragments::FragmentId;
use capi_process::InstructionAddress;
use capi_protocol::updates::Code;

#[derive(Clone, Debug, Default)]
pub struct DebugCode {
    pub inner: Option<Code>,
}

impl DebugCode {
    pub fn get(&self) -> anyhow::Result<&Code> {
        self.inner
            .as_ref()
            .ok_or_else(|| anyhow!("Code is not available yet."))
    }

    pub fn fragment_to_instructions(
        &self,
        fragment: &FragmentId,
    ) -> anyhow::Result<InstructionAddress> {
        let code = self.get()?;
        code.source_map
            .fragment_to_instructions(fragment)
            .first()
            .copied()
            .ok_or_else(|| anyhow!("Fragment does not map to instruction."))
    }
}
