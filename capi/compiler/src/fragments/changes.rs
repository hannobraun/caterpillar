use super::Function;

#[derive(Debug)]
pub struct Changes {
    pub added: Vec<Function>,
    pub updated: Vec<(Function, Function)>,
}
