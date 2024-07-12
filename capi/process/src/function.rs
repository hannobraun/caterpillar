use crate::instructions::Instructions;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub instructions: Instructions,
}

impl Function {
    pub fn new(name: String, arguments: Vec<String>) -> Self {
        Self {
            name,
            arguments,
            instructions: Instructions::default(),
        }
    }
}
