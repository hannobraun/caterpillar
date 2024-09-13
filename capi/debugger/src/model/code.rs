use anyhow::anyhow;
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
}
