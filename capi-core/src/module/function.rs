use crate::{repr::eval::fragments::FragmentId, value};

#[derive(Clone, Debug)]
pub struct Function {
    pub name: FunctionName,
    pub body: value::Block,
}

#[derive(Clone, Debug)]
pub struct FunctionName {
    pub value: String,
    pub fragment: Option<FragmentId>,
}
