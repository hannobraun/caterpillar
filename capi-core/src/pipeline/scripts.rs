use std::collections::BTreeMap;

use crate::value;

pub type Scripts = BTreeMap<Vec<value::Symbol>, String>;
