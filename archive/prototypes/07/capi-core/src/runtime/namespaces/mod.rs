mod namespace;

pub use self::namespace::{ItemInModule, Namespace, ResolveError};

// I'm working on a larger refactoring (adding evaluation of the top-level
// context to the pipeline, to eventually simplify the update code; there's a
// big comment over there somewhere). I would like to use this space, to put
// down some thoughts on how that refactoring relates to this module, for later
// reference.
//
// Two observations:
//
// - This module is called `namespace`. It used to be called `module`, but that
//   wasn't accurate at the time, so it was renamed. I now believe that we need
//   `module` again, but namespace is an orthogonal concept that needs to be
//   kept.
// - This module is under the `runtime` module, but the new `module` module that
//   I'd like to extract from it would be the result of the compile-time
//   evaluation of the top-level context. It needs to go somewhere else.
//
// So, I plan to extract a module called `module` (or maybe `modules`), and I
// think it will define a data structure that will be populated by the compile-
// time evaluation. It will contain functions, tests, eventually sub-modules.
//
// As of this writing, this data structure basically exists, in the form of
// `UserDefinedItems`. I suspect I will rename it to `Module`, move it to the
// new `module` module, and that will be the start of that process.
//
// This will leave the `namespace` module as something that owns the
// user-defined `Module`, while adding intrinsics and platform functions into
// the mix. Which is probably where it needs to be for now.
//
// That leaves the concepts a bit muddled though, at least as long as there
// really only is one global module, that everything is automatically imported
// into. Eventually, I suspect that a namespace will be something that will be
// applied to a given model, according to what has been imported into the
// namespace.
