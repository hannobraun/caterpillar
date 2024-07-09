use super::FragmentId;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FragmentAddress {
    pub parent: FragmentAddressParent,
    pub next: Option<FragmentId>,
}

impl FragmentAddress {
    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        self.parent.hash(hasher);
        if let Some(next) = self.next {
            hasher.update(next.hash.as_bytes());
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum FragmentAddressParent {
    Function { name: String },
}

impl FragmentAddressParent {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        let FragmentAddressParent::Function { name } = self;
        hasher.update(name.as_bytes());
    }
}
