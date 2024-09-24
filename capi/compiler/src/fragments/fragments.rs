use std::ops::{Deref, DerefMut};

use super::{FragmentId, FragmentMap};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    /// # The first fragment in the root context
    ///
    /// This indirectly points to all existing fragments.
    pub root: Option<FragmentId>,

    pub map: FragmentMap,
}

impl Deref for Fragments {
    type Target = FragmentMap;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Fragments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
