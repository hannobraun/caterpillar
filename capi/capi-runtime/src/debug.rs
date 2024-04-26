#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugEvent {
    ToggleBreakpoint { location: SourceLocation },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SourceLocation {
    pub function: String,
    pub line: u32,
}

impl SourceLocation {
    pub fn first_in_function(function: String) -> Self {
        Self { function, line: 0 }
    }

    pub fn increment(&mut self) -> Self {
        let self_ = self.clone();
        self.line += 1;
        self_
    }
}
