use std::{collections::HashMap, fmt};

use super::value::Value;

#[derive(Debug)]
pub struct Syntax {
    fragments: HashMap<blake3::Hash, (SyntaxHandle, SyntaxFragment)>,
    generation: u64,
}

impl Syntax {
    pub fn new() -> Self {
        Self {
            fragments: HashMap::new(),
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
        if let Some((_, existing)) = self.fragments.get(&handle.hash) {
            // We can't just compare the two fragments directly here, as the
            // generation is allowed to be different. We have to go into a bit
            // of extra effort to make it work.

            // The payload might be a block, in which case the generation of its
            // `SyntaxHandle` would make it into a regular old comparison.
            match (&existing.payload, &fragment.payload) {
                (
                    SyntaxElement::Value(Value::Block(existing)),
                    SyntaxElement::Value(Value::Block(new)),
                ) => {
                    assert_eq!(existing.map(hash), new.map(hash));
                }
                (SyntaxElement::Value(existing), SyntaxElement::Value(new)) => {
                    assert_eq!(existing, new)
                }
                (SyntaxElement::Word(existing), SyntaxElement::Word(new)) => {
                    assert_eq!(existing, new)
                }
                (SyntaxElement::Value(_), SyntaxElement::Word(_))
                | (SyntaxElement::Word(_), SyntaxElement::Value(_)) => {
                    panic!("Hash collision!")
                }
            }

            // Here we are comparing handles, whose generation is allowed to be
            // different.
            assert_eq!(existing.next.map(hash), fragment.next.map(hash));

            fn hash(handle: SyntaxHandle) -> blake3::Hash {
                handle.hash
            }
        }

        self.fragments.insert(handle.hash, (handle, fragment));

        handle
    }

    pub fn get(&self, handle: SyntaxHandle) -> SyntaxFragment {
        // This shouldn't ever panic, as we currently only ever add fragments,
        // never remove them, and only ever create handles for fragments we add.
        self.fragments.get(&handle.hash).cloned().unwrap().1
    }

    pub fn find_replaced_fragments(
        &self,
    ) -> Vec<(
        (SyntaxHandle, SyntaxFragment),
        (SyntaxHandle, SyntaxFragment),
    )> {
        let old_fragments = self.fragments.values().filter(|(_, fragment)| {
            match fragment.next {
                Some(handle) => handle.generation != self.generation,
                None => false,
            }
        });

        let mut replaced_fragments = Vec::new();
        for (handle, fragment) in old_fragments {
            let mut potential_replacements =
                self.fragments
                    .values()
                    .filter(|(_, potential_replacement)| {
                        match (fragment.next, potential_replacement.next) {
                            (Some(a), Some(b)) => {
                                a.hash == b.hash && a.generation < b.generation
                            }
                            _ => false,
                        }
                    });

            if let Some(replacement) = potential_replacements.next() {
                replaced_fragments
                    .push(((*handle, fragment.clone()), replacement.clone()));
            }

            assert!(potential_replacements.next().is_none());
        }

        replaced_fragments
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SyntaxHandle {
    pub hash: blake3::Hash,
    pub generation: u64,
}

impl fmt::Display for SyntaxHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The string representation of `SyntaxHandle` is used for hashing, so
        // the generation, which must not influence the hash, must not show up
        // here.
        write!(f, "{}", self.hash)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxFragment {
    pub token_range: TokenRange,
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
pub struct TokenRange {
    pub start: blake3::Hash,
    pub end: blake3::Hash,
}

impl TokenRange {
    pub fn one(hash: blake3::Hash) -> Self {
        Self {
            start: hash,
            end: hash,
        }
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
