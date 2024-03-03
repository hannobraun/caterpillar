use crate::repr::eval::value;

#[derive(Clone, Debug)]
pub struct Function {
    pub name: FunctionName,
    pub body: value::Block,
}

#[derive(Clone, Debug)]
pub struct FunctionName {
    pub value: String,
}
