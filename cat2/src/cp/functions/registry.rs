use std::collections::BTreeMap;

use super::{Args, Function};

pub type Registry = BTreeMap<(String, Args), Function>;
