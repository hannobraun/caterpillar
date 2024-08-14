use std::fmt;

use capi_process::Value;

use super::Function;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FragmentExpression {
    BindingDefinitions {
        names: Vec<String>,
    },
    Function {
        function: Function,
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

    /// # A call to a user-defined function
    ///
    /// ## Implementation Note
    ///
    /// This enum variant references the function by name. It should instead
    /// reference the function using an `id: FragmentId` field.
    ///
    /// This would have the advantage of versioning this call. It could refer to
    /// any available version of the function, which is a useful feature to have
    /// for many reasons.
    ///
    /// Unfortunately, this is not easy. There are two main hurdles, as best I
    /// can tell:
    ///
    /// 1. It requires function fragments to be created in the correct order, as
    ///    the called function must be created before its caller.
    /// 2. There would need to be special handling of recursive calls, or there
    ///    would be a dependency cycle when hashing the calls and their targets.
    ///
    /// I think what we need, is a new compiler pass that creates a call graph.
    /// This call graph can then be used to order the creation of fragments,
    /// from the leaves up, as well as to detect any recursive call cycles.
    ///
    /// As for the handling of those, here is some information on how Unison
    /// does that, which might prove useful:
    /// https://stackoverflow.com/a/73343072/8369834
    ResolvedFunction {
        name: String,

        /// Indicate whether the call is in tail position
        ///
        /// This flag is relevant for tail call elimination. It is only required
        /// for calls to user-defined functions, as only those require compile-
        /// time tail call elimination:
        ///
        /// - Built-in functions are expected to perform their own tail call
        ///   elimination at runtime, if necessary.
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
            Self::Function { .. } => write!(f, "block"),
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
