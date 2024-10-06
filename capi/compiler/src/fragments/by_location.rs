use super::FragmentId;

/// # Code fragments, tracked by their location
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct FragmentsByLocation {}

impl FragmentsByLocation {
    /// # Insert a fragment
    pub fn insert(
        &mut self,
        id: FragmentId,
        previous: Option<FragmentId>,
        next: Option<FragmentId>,
    ) {
        // This is just a placeholder so far.
        let _ = id;
        let _ = previous;
        let _ = next;
    }
}
