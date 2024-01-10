use std::collections::BTreeMap;

use super::UserDefinedFunction;

#[derive(Debug)]
pub struct Functions(pub BTreeMap<String, UserDefinedFunction>);
