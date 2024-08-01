use super::{FragmentExpression, FragmentId};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragment {
    pub parent: Option<FragmentParent>,
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
        match self.payload {
            FragmentPayload::Expression { next, .. } => Some(next),
            FragmentPayload::Function { next, .. } => Some(next),
            FragmentPayload::Terminator => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FragmentParent {
    Fragment { id: FragmentId },
    Function { name: String },
}

impl FragmentParent {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            FragmentParent::Function { name } => {
                hasher.update(b"function");
                hasher.update(name.as_bytes());
            }
            FragmentParent::Fragment { id } => {
                hasher.update(b"fragment");
                id.hash(hasher);
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FragmentPayload {
    Expression {
        expression: FragmentExpression,
        next: FragmentId,
    },
    Function {
        name: String,
        args: Vec<String>,
        start: FragmentId,
        next: FragmentId,
    },
    Terminator,
}

impl FragmentPayload {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            Self::Expression { expression, next } => {
                hasher.update(b"expression");
                expression.hash(hasher);
                next.hash(hasher);
            }
            Self::Function {
                name,
                args,
                start,
                next,
            } => {
                hasher.update(b"function");
                hasher.update(name.as_bytes());
                for arg in args {
                    hasher.update(arg.as_bytes());
                }
                start.hash(hasher);
                next.hash(hasher);
            }
            Self::Terminator => {
                hasher.update(b"terminator");
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub start: FragmentId,
}
