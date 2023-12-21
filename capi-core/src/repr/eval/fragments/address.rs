use super::FragmentId;

/// Uniquely identifies the location of a code fragment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FragmentAddress {
    /// The ID of this fragment's parent (which is also a fragment), if any
    ///
    /// The parent is `None` for all fragments within the top-level context of
    /// a script. For all fragments within blocks, it points to the fragment
    /// *after* the block (the block's `next` fragment).
    ///
    /// Using the block's `next` fragment as a parent is done for practical
    /// reasons, as the ID of the next fragment is available when a parent ID is
    /// needed, while the ID of the block itself (or the fragment before it)
    /// isn't. And they can't be made available either, as they depend on the
    /// block contents, which would result in a circular dependency.
    ///
    /// Please note that this is also true for blocks, which are the last syntax
    /// element in their respective context. [Terminator] fragments are added to
    /// each context, to make sure that a block is *never* the last fragment,
    /// even if it is the last syntactic element.
    ///
    /// [Terminator]: crate::repr::eval::fragments::FragmentPayload::Terminator
    pub parent: Option<FragmentId>,

    /// The ID of the fragment that comes after this one, if any
    ///
    /// This can only be `None`, if the fragment is a [terminator][Terminator].
    /// Otherwise, this always refers to the next fragment, which might be a
    /// terminator, if the fragment was created from the last syntax element in
    /// a given context.
    ///
    /// [Terminator]: crate::repr::eval::fragments::FragmentPayload::Terminator
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
