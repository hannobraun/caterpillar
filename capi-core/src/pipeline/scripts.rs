use std::collections::BTreeMap;

use crate::value;

#[derive(Default)]
pub struct Scripts {
    // Eventually, I'm going to need to store the path of the entry script here,
    // so the `Interpreter` always knows where to start the pipeline.
    //
    // (Actually, once we store the entry script path here, we don't need to
    // pass its code in a separate variable at all.)
    pub inner: BTreeMap<ScriptPath, String>,
}

pub type ScriptPath = Vec<value::Symbol>;
