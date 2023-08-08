use std::{collections::HashMap, fmt};

use crate::value::Value;

#[derive(Debug)]
pub struct Syntax {
    inner: HashMap<SyntaxHandle, SyntaxFragment>,
    generation: u64,
}

impl Syntax {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            generation: 0,
        }
    }

    pub fn prepare_update(&mut self) {
        self.generation += 1;
    }

    pub fn add(&mut self, fragment: SyntaxFragment) -> SyntaxHandle {
        let handle = SyntaxHandle {
            hash: fragment.next_hash(),
            generation: self.generation,
        };

        // A hash collision should be exceedingly unlikely, but I'm not sure
        // quite *how* unlikely. Also, I don't fully trust my code to work
        // perfectly.
        //
        // Let's make sure, just for now, there actually are no hash collisions,
        // okay?
        if let Some(existing) = self.inner.get(&handle) {
            assert_eq!(existing, &fragment);
        }

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
    hash: blake3::Hash,
    generation: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxFragment {
    pub payload: SyntaxElement,
    pub next: Option<SyntaxHandle>,
}

impl SyntaxFragment {
    fn next_hash(&self) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();

        hasher.update(self.payload.to_string().as_bytes());
        if let Some(next) = self.next {
            hasher.update(next.hash.as_bytes());
        }

        hasher.finalize()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntaxElement {
    Value(Value),
    Word(String),
}

impl fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SyntaxElement::Value(value) => write!(f, "{value}"),
            SyntaxElement::Word(word) => write!(f, "{word}"),
        }
    }
}
