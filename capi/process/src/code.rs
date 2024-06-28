use std::collections::BTreeMap;

use crate::Function;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Code {
    pub functions: BTreeMap<String, Function>,
}
