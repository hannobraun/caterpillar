use capi_process::{InstructionIndex, Location};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Command {
    BreakpointClear { instruction: InstructionIndex },
    BreakpointSet { location: InstructionIndex },
    Continue { and_stop_at: Option<Location> },
    Reset,
    Step,
    Stop,
}

impl Command {
    pub fn deserialize(bytes: SerializedCommand) -> Self {
        let string = std::str::from_utf8(&bytes).unwrap();
        ron::from_str(string).unwrap()
    }

    pub fn serialize(&self) -> SerializedCommand {
        ron::to_string(self).unwrap().into_bytes()
    }
}

pub type SerializedCommand = Vec<u8>;
