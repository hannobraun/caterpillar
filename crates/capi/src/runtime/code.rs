use std::collections::BTreeMap;

use super::Function;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Code {
    pub functions: BTreeMap<String, Function>,
}
