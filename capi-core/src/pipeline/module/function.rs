use crate::repr::eval::value;

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub body: value::Block,
}
