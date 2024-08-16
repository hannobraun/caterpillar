use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Intrinsic {
    Eval,
}

impl fmt::Display for Intrinsic {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Intrinsic::Eval => write!(_f, "eval"),
        }
    }
}
