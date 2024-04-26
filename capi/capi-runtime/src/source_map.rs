use std::collections::BTreeMap;

use crate::LineLocation;

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct SourceMap {
    pub address_to_location: BTreeMap<usize, LineLocation>,
}
