use std::collections::BTreeMap;

use crate::{InstructionAddress, Instructions};

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

    /// # The arguments of each function
    ///
    /// ## Implementation Note
    ///
    /// This is used by the evaluator to perform function calls, but it is not
    /// really desirable to do it this way. There's really no good reason for
    /// the evaluator to know about the names of functions, except that the
    /// current implementation requires it.
    ///
    /// This is tied to how bindings are handled: Those are also known by name
    /// to the evaluator, and there's no good reason for that, except that it's
    /// how it works right now.
    ///
    /// Eventually, the compiler should get smarter, and place bindings at fixed
    /// addresses within a stack frame. Then it can generate the right
    /// instruction to get the evaluator to do the right things, and no more
    /// names need to be known at runtime.
    ///
    /// Once that has been achieved, this field can be removed. (And maybe this
    /// whole struct along with it, to be replaced by `Instructions`. But that's
    /// another topic.)
    pub function_arguments: BTreeMap<InstructionAddress, Vec<String>>,
}
