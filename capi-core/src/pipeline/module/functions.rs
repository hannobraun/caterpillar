use std::collections::BTreeMap;

use crate::repr::eval::value;

#[derive(Debug, Default)]
pub struct Functions(pub BTreeMap<String, Function>);

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub body: value::Block,
}
