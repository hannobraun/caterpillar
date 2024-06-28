use std::collections::BTreeMap;

use crate::Function;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Code {
    pub functions: BTreeMap<String, Function>,
}
