use capi_process::InstructionAddress;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum CommandToRuntime {
    BreakpointClear { instruction: InstructionAddress },
    BreakpointSet { instruction: InstructionAddress },
    Continue,
    Reset,
    Step,
    Stop,
}

impl CommandToRuntime {
    pub fn deserialize(bytes: SerializedCommandToRuntime) -> Self {
        let string = std::str::from_utf8(&bytes).unwrap();
        ron::from_str(string).unwrap()
    }

    pub fn serialize(&self) -> SerializedCommandToRuntime {
        ron::to_string(self).unwrap().into_bytes()
    }
}

pub type SerializedCommandToRuntime = Vec<u8>;
