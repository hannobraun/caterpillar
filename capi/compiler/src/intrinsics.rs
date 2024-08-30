use std::fmt;

/// # Special functions that are known to the compiler
///
/// When encountering a call to an intrinsic, the compiler will directly
/// translate that into the appropriate instructions.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Intrinsic {
    /// # Add two signed 8-bit integers
    AddS8,

    /// # Trigger a breakpoint
    Brk,

    /// # Evaluate an anonymous function
    Eval,
}

impl Intrinsic {
    pub fn from_name(name: &str) -> Option<Intrinsic> {
        let intrinsic = match name {
            "add_s8" => Self::AddS8,
            "brk" => Self::Brk,
            "eval" => Self::Eval,

            _ => {
                return None;
            }
        };

        Some(intrinsic)
    }
}

impl fmt::Display for Intrinsic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Intrinsic::AddS8 => write!(f, "add_s8"),
            Intrinsic::Brk => write!(f, "brk"),
            Intrinsic::Eval => write!(f, "eval"),
        }
    }
}
