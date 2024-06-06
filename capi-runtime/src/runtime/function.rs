use std::collections::VecDeque;

use super::Location;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub arguments: Vec<String>,
    pub instructions: VecDeque<Location>,
}

impl Function {
    pub fn new(arguments: Vec<String>) -> Self {
        Self {
            arguments,
            instructions: VecDeque::new(),
        }
    }
}
