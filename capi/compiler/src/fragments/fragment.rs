use super::{hash::FragmentHash, FragmentId, Payload};

/// # A content-addressed piece of code
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Fragment {
    /// # This fragment's parent
    ///
    /// Refers to the fragment that is the parent of this fragment. If this
    /// fragment resides in the root context, then is has no parent.
    ///
    /// All other fragments have a parent. By convention, this is the fragment
    /// _after_ the function that this fragment resides in (i.e. the `next`
    /// fragment of that function).
    ///
    /// This must be so, because by the time that a fragment is constructed, the
    /// function fragment for the function it resides in, or any fragments
    /// preceding that, are not constructed yet. Thus, they do not have an ID
    /// that can be used to refer to them.
    ///
    /// Any _succeeding_ fragments, on the other hand, are already constructed.
    /// Therefore, the `next` fragment of the function fragment can stand in as
    /// the parent.
    ///
    /// Function fragments always have a `next` fragment that can be used in
    /// this way. This is that reason that terminators exist, to make sure of
    /// that.
    pub parent: Option<FragmentId>,

    pub kind: FragmentKind,
}

impl Fragment {
    pub fn id(&self) -> FragmentId {
        let mut hasher = blake3::Hasher::new();
        self.hash(&mut hasher);
        FragmentId::new(hasher.finalize())
    }

    pub fn next(&self) -> Option<FragmentId> {
        match &self.kind {
            FragmentKind::Payload { next, .. } => Some(*next),
            FragmentKind::Terminator => None,
        }
    }

    pub fn is_comment(&self) -> bool {
        let FragmentKind::Payload {
            payload: Payload::Comment { .. },
            ..
        } = &self.kind
        else {
            return false;
        };

        true
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FragmentKind {
    /// # This fragment carries a payload
    Payload {
        /// # The payload that the fragment carries
        payload: Payload,

        /// # The next fragment after this one
        ///
        /// Every fragment resides in a context, either a function or the root
        /// context. Every payload-carrying fragment has a fragment that follows
        /// it within that context, which is either another payload-carrying
        /// fragment, or a terminator.
        next: FragmentId,
    },

    /// # This fragment is a terminator
    ///
    /// Terminators carry no payload and, well, terminate any context in which
    /// fragments reside. The reason for this is explained in the documentation
    /// of [`Fragment`]'s `parent` field.
    Terminator,
}
