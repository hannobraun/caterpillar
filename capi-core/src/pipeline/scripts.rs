use std::collections::BTreeMap;

use crate::repr::eval::value;

#[derive(Debug, Default)]
pub struct Scripts {
    pub entry_script_path: ScriptPath,
    pub inner: BTreeMap<ScriptPath, String>,
}

/// The path of a script, represented as a series of symbols
///
/// This is a platform-independent way to represent the script path. We can't
/// use an actual `Path` (or `PathBuf`, rather), because there might not be a
/// file system available where we're running this (in the browser, for
/// example).
pub type ScriptPath = Vec<value::Symbol>;
