use crate::{Effect, Stack};

pub fn builtin_by_name(_name: &str) -> Option<Builtin> {
    None
}

pub type Builtin = fn(&mut Stack) -> Result;

pub type Result = std::result::Result<(), Effect>;
