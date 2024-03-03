use std::collections::BTreeMap;

use super::Function;

#[derive(Debug, Default)]
pub struct Functions(pub BTreeMap<String, Function>);
