use std::fmt;

/// # Special functions that are known to the compiler
///
/// When encountering a call to an intrinsic, the compiler will directly
/// translate that into the appropriate instructions.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Intrinsic {
    /// # Trigger a breakpoint
    Brk,

    /// # Evaluate an anonymous function
    Eval,
}

impl Intrinsic {
    pub fn from_name(name: &str) -> Option<Intrinsic> {
        match name {
            "brk" => Some(Self::Brk),
            "eval" => Some(Self::Eval),
            _ => None,
        }
    }
}

impl fmt::Display for Intrinsic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Intrinsic::Brk => write!(f, "brk"),
            Intrinsic::Eval => write!(f, "eval"),
        }
    }
}
