use std::collections::VecDeque;

use super::Location;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub instructions: VecDeque<Location>,
}

impl Function {
    pub fn new(name: String, arguments: Vec<String>) -> Self {
        Self {
            name,
            arguments,
            instructions: VecDeque::new(),
        }
    }
}
