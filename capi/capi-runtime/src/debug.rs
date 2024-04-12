#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugEvent {
    ToggleBreakpoint { location: LineLocation },
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LineLocation {
    pub function: String,
    pub line: u32,
}
