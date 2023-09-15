use super::{FragmentAddress, FragmentId, FragmentPayload};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fragment {
    pub(super) address: FragmentAddress,
    pub payload: FragmentPayload,
}

impl Fragment {
    pub fn new(address: FragmentAddress, payload: FragmentPayload) -> Self {
        Self { address, payload }
    }

    pub fn id(&self) -> FragmentId {
        let hash = self.hash();
        FragmentId { hash }
    }

    pub fn next(&self) -> Option<FragmentId> {
        self.address.next
    }

    pub(super) fn hash(&self) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();

        self.address.hash(&mut hasher);
        self.payload.hash(&mut hasher);

        hasher.finalize()
    }
}
