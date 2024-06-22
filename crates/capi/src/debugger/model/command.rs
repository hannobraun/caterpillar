use crate::runtime;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugCommand {
    BreakpointClear {
        location: runtime::Location,
    },
    BreakpointSet {
        location: runtime::Location,
    },
    Continue {
        and_stop_at: Option<runtime::Location>,
    },
    Reset,
    Step,
    Stop,
}

impl DebugCommand {
    pub fn deserialize(bytes: SerializedCommand) -> Self {
        let string = std::str::from_utf8(&bytes).unwrap();
        ron::from_str(string).unwrap()
    }

    pub fn serialize(&self) -> SerializedCommand {
        ron::to_string(self).unwrap().into_bytes()
    }
}

pub type SerializedCommand = Vec<u8>;
