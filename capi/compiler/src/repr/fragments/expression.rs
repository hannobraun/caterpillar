use std::fmt;

use capi_process::Value;

use super::FragmentId;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FragmentExpression {
    BindingDefinitions { names: Vec<String> },
    BindingEvaluation { name: String },
    Block { start: FragmentId },
    BuiltinCall { name: String },
    Comment { text: String },
    FunctionCall { name: String },
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
            Self::BindingEvaluation { name } => {
                hasher.update(b"binding evaluation");
                hasher.update(name.as_bytes());
            }
            Self::Block { start } => {
                hasher.update(b"block");
                start.hash(hasher);
            }
            Self::BuiltinCall { name } => {
                hasher.update(b"builtin call");
                hasher.update(name.as_bytes());
            }
            Self::Comment { text } => {
                hasher.update(b"comment");
                hasher.update(text.as_bytes());
            }
            Self::FunctionCall { name } => {
                hasher.update(b"function call");
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
                writeln!(f, " .")
            }
            Self::BindingEvaluation { name } => writeln!(f, "{name}"),
            Self::Block { start } => writeln!(f, "block@{start}"),
            Self::BuiltinCall { name } => writeln!(f, "{name}"),
            Self::Comment { text } => writeln!(f, "# {text}"),
            Self::FunctionCall { name } => write!(f, "{name}"),
            Self::Value(value) => write!(f, "{value}"),
        }
    }
}
