use capi_compiler::{repr::fragments::Fragments, source_map::SourceMap};
use capi_process::{GameEngineHost, Process};

use crate::memory::Memory;

#[allow(clippy::large_enum_variant)] // haven't optimized this yet
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Update {
    SourceCode(SourceCode),
    Process(Process<GameEngineHost>),
    Memory { memory: Memory },
}

impl Update {
    pub fn deserialize(bytes: SerializedUpdate) -> Self {
        let string = std::str::from_utf8(&bytes).unwrap();
        ron::from_str(string).unwrap()
    }

    pub fn serialize(&self) -> SerializedUpdate {
        ron::to_string(self).unwrap().into_bytes()
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SourceCode {
    pub fragments: Fragments,
    pub source_map: SourceMap,
}

pub type SerializedUpdate = Vec<u8>;
