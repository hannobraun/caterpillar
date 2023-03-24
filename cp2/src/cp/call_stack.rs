use std::collections::BTreeMap;

use super::{data_stack::Value, Expressions};

pub type Bindings = BTreeMap<String, Value>;
pub type Functions = BTreeMap<String, Expressions>;
