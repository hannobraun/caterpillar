#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location {
    function: String,
    index: u32,
}

impl Location {
    pub fn function(&self) -> &str {
        &self.function
    }

    pub fn first_in_function(function: String) -> Self {
        Self { function, index: 0 }
    }

    pub fn increment(&mut self) -> Self {
        let self_ = self.clone();
        self.index += 1;
        self_
    }
}
