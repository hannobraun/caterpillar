use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Intrinsic {
    Eval,
}

impl fmt::Display for Intrinsic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Intrinsic::Eval => write!(f, "eval"),
        }
    }
}
