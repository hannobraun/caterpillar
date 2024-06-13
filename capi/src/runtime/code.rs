use std::collections::BTreeMap;

use crate::runtime;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Code {
    pub functions: BTreeMap<String, runtime::Function>,
}
