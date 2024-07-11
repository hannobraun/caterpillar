use super::{FragmentId, FragmentPayload};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragment {
    pub address: FragmentAddress,
    pub payload: FragmentPayload,
}

impl Fragment {
    pub fn id(&self) -> FragmentId {
        let mut hasher = blake3::Hasher::new();

        self.address.parent.hash(&mut hasher);
        if let Some(next) = self.address.next {
            hasher.update(next.hash.as_bytes());
        };
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
