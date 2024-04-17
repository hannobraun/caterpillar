use std::collections::BTreeMap;

use crate::LineLocation;

#[derive(Clone, Default)]
pub struct SourceMap {
    pub inner: BTreeMap<usize, LineLocation>,
}
