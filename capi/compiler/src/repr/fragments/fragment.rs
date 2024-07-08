use crate::repr::syntax::Location;

use super::{FragmentAddress, FragmentId, FragmentPayload};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragment {
    pub address: FragmentAddress,
    pub payload: FragmentPayload,
    pub location: Location,
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
