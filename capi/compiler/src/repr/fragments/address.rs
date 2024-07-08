use super::FragmentId;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FragmentAddress {
    pub function: String,
    pub next: Option<FragmentId>,
}

impl FragmentAddress {
    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        hasher.update(self.function.as_bytes());
        if let Some(next) = self.next {
            hasher.update(next.hash.as_bytes());
        }
    }
}
