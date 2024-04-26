use crate::SourceLocation;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugEvent {
    ToggleBreakpoint { location: SourceLocation },
}
