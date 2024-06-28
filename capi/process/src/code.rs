use std::collections::BTreeMap;

use crate::Function;

/// Code that is executed by the interpreter
///
/// # Implementation Note
///
/// This isn't quite actual bytecode yet, but something more high-level and less
/// efficient. I needed a name here to better distinguish it from source code
/// though, and since turning this into real bytecode is the idea, I figured why
/// not just it as the name, even though it doesn't fit perfectly.
#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Bytecode {
    pub functions: BTreeMap<String, Function>,
}

impl Bytecode {
    pub fn entry(&self) -> Option<Function> {
        self.functions.get("main").cloned()
    }
}
