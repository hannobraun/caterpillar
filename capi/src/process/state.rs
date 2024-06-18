#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum ProcessState {
    #[default]
    Running,

    Finished,
}

impl ProcessState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }

    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Finished)
    }
}
