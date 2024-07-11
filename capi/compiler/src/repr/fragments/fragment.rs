use super::{FragmentId, FragmentPayload};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragment {
    pub address: FragmentAddress,
    pub payload: FragmentPayload,
}

impl Fragment {
    pub fn id(&self) -> FragmentId {
        let mut hasher = blake3::Hasher::new();

        self.address.hash(&mut hasher);
        self.payload.hash(&mut hasher);

        FragmentId {
            hash: hasher.finalize(),
        }
    }
}

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
