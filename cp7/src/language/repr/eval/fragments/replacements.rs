use std::collections::HashMap;

use super::FragmentId;

#[derive(Debug)]
pub struct Replacements {
    pub(super) inner: HashMap<FragmentId, FragmentId>,
}
