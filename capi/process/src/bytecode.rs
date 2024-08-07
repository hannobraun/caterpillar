use crate::Instructions;

/// Code that is executed by the interpreter
///
/// # Implementation Note
///
/// This isn't quite actual bytecode yet, but something more high-level and less
/// efficient. I needed a name here to better distinguish it from source code
/// though, and since turning this into real bytecode is the idea, I figured why
/// not just use that as the name, even though it doesn't fit perfectly yet.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Bytecode {
    pub instructions: Instructions,
}
