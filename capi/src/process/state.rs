#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State {
    pub has_finished: bool,
}

impl State {
    pub fn is_running(&self) -> bool {
        !self.has_finished
    }

    pub fn is_finished(&self) -> bool {
        self.has_finished
    }
}
