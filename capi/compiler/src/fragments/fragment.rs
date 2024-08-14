use super::{FragmentExpression, FragmentId, Function};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragment {
    /// The parent of this fragment
    ///
    /// Points to the fragment that is the parent of this fragment. If this
    /// fragment resides in the root context, then this is `None`.
    ///
    /// Otherwise, this fragment must reside in a function or block. In this
    /// case, by convention, this points to the first fragment _after_ the
    /// function or block (i.e. its next fragment).
    ///
    /// This must be so, because the block or function itself, or any fragments
    /// preceding it, are not complete yet, and thus do not have a hash. Their
    /// hash depends on the hashes of the fragments they contain, which in turn
    /// depend on their parents. By making the function's or block's next
    /// fragment the parent, a circular dependency is avoided.
    pub parent: Option<FragmentId>,

    pub payload: FragmentPayload,
}

impl Fragment {
    pub fn id(&self) -> FragmentId {
        let mut hasher = blake3::Hasher::new();

        if let Some(parent) = self.parent.as_ref() {
            parent.hash(&mut hasher);
        }
        self.payload.hash(&mut hasher);

        FragmentId::new(hasher.finalize())
    }

    pub fn next(&self) -> Option<FragmentId> {
        match &self.payload {
            FragmentPayload::Function { next, .. } => Some(*next),
            FragmentPayload::Expression { next, .. } => Some(*next),
            FragmentPayload::Terminator => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FragmentPayload {
    /// A cluster is a group of functions that share a name
    ///
    /// Functions are called by cluster, and the call is dispatched to a
    /// function within the cluster using pattern matching on their arguments.
    ///
    /// For the sake of uniformity, all functions are organized into clusters,
    /// so a cluster might have only a single member.
    Function {
        function: Function,
        next: FragmentId,
    },

    Expression {
        expression: FragmentExpression,
        next: FragmentId,
    },
    Terminator,
}

impl FragmentPayload {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            Self::Function { function, next } => {
                hasher.update(b"cluster");
                function.hash(hasher);
                next.hash(hasher);
            }
            Self::Expression { expression, next } => {
                hasher.update(b"expression");
                expression.hash(hasher);
                next.hash(hasher);
            }
            Self::Terminator => {
                hasher.update(b"terminator");
            }
        }
    }
}
