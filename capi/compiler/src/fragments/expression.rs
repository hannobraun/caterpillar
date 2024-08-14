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

    /// # A call to a cluster of functions
    ///
    /// ## Implementation Note
    ///
    /// This enum variant references the cluster by name. It should instead
    /// reference the cluster using an `id: FragmentId` field.
    ///
    /// This would have the advantage of versioning this call. It could refer to
    /// any available version of the cluster, which is a useful feature to have
    /// for many reasons.
    ///
    /// Unfortunately, this is not easy. There are two main hurdles, as best I
    /// can tell:
    ///
    /// 1. It requires cluster fragments to be created in the correct order, as
    ///    the called cluster must be created before its caller.
    /// 2. There would need to be special handling of recursive calls, or there
    ///    would be a dependency cycle when hashing the calls and their targets.
    ///
    /// I think what we need, is a new compiler pass that creates a call graph.
    /// This call graph can then be used to order the creating of fragments,
    /// from the leaves up, as well as to detect recursive call cycles.
    ///
    /// As for the handling of those, here is some information on how Unison
    /// does that, which might prove useful:
    /// https://stackoverflow.com/a/73343072/8369834
    ResolvedFunction {
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
            Self::ResolvedFunction { name, is_tail_call } => {
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
            Self::ResolvedFunction { name, .. } => write!(f, "{name}"),
            Self::ResolvedHostFunction { name } => write!(f, "{name}"),
            Self::UnresolvedIdentifier { name } => write!(f, "{name}"),
            Self::Value(value) => write!(f, "{value}"),
        }
    }
}
