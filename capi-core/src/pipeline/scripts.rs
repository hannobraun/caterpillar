use std::collections::BTreeMap;

use crate::value;

#[derive(Debug, Default)]
pub struct Scripts {
    // Eventually, I'm going to need to store the path of the entry script here,
    // so the `Interpreter` always knows where to start the pipeline.
    //
    // (Actually, once we store the entry script path here, we don't need to
    // pass its code in a separate variable at all.)
    pub inner: BTreeMap<ScriptPath, String>,
}

/// The path of a script, represented as a series of symbols
///
/// This is a platform-independent way to represent the script path. We can't
/// use an actual `Path` (or `PathBuf`, rather), because there might not be a
/// file system available where we're running this (in the browser, for
/// example).
pub type ScriptPath = Vec<value::Symbol>;
