use super::{FragmentAddress, FragmentId, FragmentPayload};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fragment {
    pub address: FragmentAddress,
    pub payload: FragmentPayload,
}

impl Fragment {
    pub fn new(address: FragmentAddress, payload: FragmentPayload) -> Self {
        Self { address, payload }
    }

    pub fn id(&self) -> FragmentId {
        let hash = {
            let mut hasher = blake3::Hasher::new();

            self.address.hash(&mut hasher);
            self.payload.hash(&mut hasher);

            hasher.finalize()
        };

        FragmentId { hash }
    }

    pub fn next(&self) -> Option<FragmentId> {
        self.address.next
    }
}
