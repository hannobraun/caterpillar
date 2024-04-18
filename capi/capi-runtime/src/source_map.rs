use std::collections::BTreeMap;

use crate::LineLocation;

#[derive(Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct SourceMap {
    pub inner: BTreeMap<usize, LineLocation>,
}
