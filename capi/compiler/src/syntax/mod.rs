//! # The representation of the code before it is compiled into fragments
//!
//! ## Implementation Note
//!
//! After the latest round of cleanups in `fragments`, this representation has
//! grown awfully close to `fragments`. To the point that I'm starting to
//! believe they should be merged.
//!
//! I'm sure there are some details to work out there, but it definitely _looks_
//! feasible. Since `fragments` is a bit more advanced, in the ways it can be
//! queried, for example, this module should get merged into that one.
//!
//! Maybe the resulting merged module should just be called `code`, since that's
//! what it is. The compiler's representation of source code. And once we have a
//! code database, it's also going to be the canonical representation of source
//! code.

mod expression;
mod function;

pub use self::{
    expression::{Expression, IdentifierTarget},
    function::{Branch, Function, Pattern},
};
