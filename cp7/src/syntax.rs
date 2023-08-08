use std::collections::HashMap;

use crate::value::Value;

pub struct Syntax {
    inner: HashMap<SyntaxHandle, SyntaxFragment>,
    next_id: u64,
}

impl Syntax {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add(&mut self, fragment: SyntaxFragment) -> SyntaxHandle {
        let id = self.next_id;
        self.next_id += 1;

        let handle = SyntaxHandle { id };
        self.inner.insert(handle, fragment);

        handle
    }

    pub fn get(&self, handle: SyntaxHandle) -> SyntaxFragment {
        // This shouldn't ever panic, as we currently only ever add fragments,
        // never remove them, and only ever create handles for fragments we add.
        self.inner.get(&handle).cloned().unwrap()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SyntaxHandle {
    // Eventually, this should be a hash of the `SyntaxFragment` that the handle
    // references, thereby making `SyntaxFragment` content-addressed. For now, a
    // simple unique ID will do.
    id: u64,
}

#[derive(Clone, Debug)]
pub struct SyntaxFragment {
    pub payload: SyntaxElement,
    pub next: Option<SyntaxHandle>,
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    Value(Value),
    Word(String),
}
