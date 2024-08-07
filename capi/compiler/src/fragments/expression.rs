use std::{collections::BTreeSet, fmt};

use capi_process::Value;

use super::FragmentId;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FragmentExpression {
    BindingDefinitions {
        names: Vec<String>,
    },
    Block {
        start: FragmentId,
        environment: BTreeSet<String>,
    },
    Comment {
        text: String,
    },
    ResolvedBinding {
        name: String,
    },
    ResolvedBuiltinFunction {
        name: String,
    },
    ResolvedCluster {
        name: String,

        /// Indicate whether the call is in tail position
        ///
        /// This flag is relevant for tail call elimination. It is only required
        /// for cluster calls, as only those require compile-time tail call
        /// elimination:
        ///
        /// - Built-in and host functions are expected to perform their own tail
        ///   call elimination at runtime, if necessary.
        /// - No other expressions can result in a new stack frame.
        is_tail_call: bool,
    },
    ResolvedHostFunction {
        name: String,
    },
    UnresolvedIdentifier {
        name: String,
    },
    Value(Value),
}

impl FragmentExpression {
    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            Self::BindingDefinitions { names } => {
                hasher.update(b"binding definition");

                for name in names {
                    hasher.update(name.as_bytes());
                }
            }
            Self::Block { start, environment } => {
                hasher.update(b"block");
                start.hash(hasher);
                for binding in environment {
                    hasher.update(binding.as_bytes());
                }
            }
            Self::Comment { text } => {
                hasher.update(b"comment");
                hasher.update(text.as_bytes());
            }
            Self::ResolvedBinding { name } => {
                hasher.update(b"resolved binding");
                hasher.update(name.as_bytes());
            }
            Self::ResolvedBuiltinFunction { name } => {
                hasher.update(b"resolved built-in function");
                hasher.update(name.as_bytes());
            }
            Self::ResolvedCluster { name, is_tail_call } => {
                hasher.update(b"resolved user function");
                hasher.update(name.as_bytes());
                hasher.update(&[(*is_tail_call).into()]);
            }
            Self::ResolvedHostFunction { name } => {
                hasher.update(b"resolved host function");
                hasher.update(name.as_bytes());
            }
            Self::UnresolvedIdentifier { name } => {
                hasher.update(b"unresolved word");
                hasher.update(name.as_bytes());
            }
            Self::Value(value) => {
                hasher.update(b"value");
                hasher.update(&value.0);
            }
        }
    }
}

impl fmt::Display for FragmentExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BindingDefinitions { names } => {
                write!(f, "=>")?;
                for name in names {
                    write!(f, " {name}")?;
                }
                write!(f, " .")
            }
            Self::Block { start, .. } => write!(f, "block@{start}"),
            Self::Comment { text } => write!(f, "# {text}"),
            Self::ResolvedBinding { name } => write!(f, "{name}"),
            Self::ResolvedBuiltinFunction { name } => write!(f, "{name}"),
            Self::ResolvedCluster { name, .. } => write!(f, "{name}"),
            Self::ResolvedHostFunction { name } => write!(f, "{name}"),
            Self::UnresolvedIdentifier { name } => write!(f, "{name}"),
            Self::Value(value) => write!(f, "{value}"),
        }
    }
}
