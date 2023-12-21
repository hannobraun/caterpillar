use super::FragmentId;

/// Uniquely identifies the location of a code fragment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FragmentAddress {
    pub parent: Option<FragmentId>,
    pub next: Option<FragmentId>,
}

impl FragmentAddress {
    pub fn display_short(&self) -> String {
        format!(
            "{{ parent: {:?}, next: {:?} }}",
            self.parent.map(|id| id.display_short()),
            self.next.map(|id| id.display_short())
        )
    }

    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        if let Some(parent) = self.parent {
            hasher.update(parent.hash.as_bytes());
        }
        if let Some(next) = self.next {
            hasher.update(next.hash.as_bytes());
        }
    }
}
