use std::{collections::HashMap, fmt};

use super::{tokens::TokenAddress, value::Value};

#[derive(Debug)]
pub struct Syntax {
    fragments: HashMap<blake3::Hash, (FragmentId, SyntaxFragment)>,
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

    pub fn add(&mut self, fragment: SyntaxFragment) -> FragmentId {
        let handle = FragmentId {
            hash: fragment.hash(),
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
                    SyntaxElement::Value(Value::Block {
                        start: existing, ..
                    }),
                    SyntaxElement::Value(Value::Block { start: new, .. }),
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
            assert_eq!(existing.next().map(hash), fragment.next().map(hash));

            fn hash(handle: FragmentId) -> blake3::Hash {
                handle.hash
            }
        }

        self.fragments.insert(handle.hash, (handle, fragment));

        handle
    }

    pub fn get(&self, handle: FragmentId) -> SyntaxFragment {
        // This shouldn't ever panic, as we currently only ever add fragments,
        // never remove them, and only ever create handles for fragments we add.
        self.fragments.get(&handle.hash).cloned().unwrap().1
    }

    pub fn find_replaced_fragments(
        &self,
    ) -> Vec<((FragmentId, SyntaxFragment), (FragmentId, SyntaxFragment))> {
        let old_fragments = self.fragments.values().filter(|(_, fragment)| {
            match fragment.next() {
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
                        match (fragment.next(), potential_replacement.next()) {
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

/// Uniquely identifies a syntax fragment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FragmentId {
    pub hash: blake3::Hash,
    generation: u64,
}

impl fmt::Display for FragmentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The string representation of `SyntaxHandle` is used for hashing, so
        // the generation, which must not influence the hash, must not show up
        // here.
        write!(f, "{}", self.hash)
    }
}

/// Uniquely identifies the location of a syntax fragment in the code
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FragmentAddress {
    // Knowing the next syntax fragment does not uniquely identify a syntax
    // fragment. Multiple functions might end in the same way, and then for the
    // syntax fragment preceding that common suffix, if one of them was changed,
    // we won't be able to tell which one.
    //
    // Knowing the previous syntax fragment doesn't help either, as different
    // functions might contain identical syntax. Then for any change in one of
    // them, we won't know where that change occurred.
    //
    // We need to know the parent of the syntax element. That will enable us to
    // uniquely identify syntax fragments in both of those cases. The question
    // is do we need to know the previous fragment in addition? Or does knowing
    // the parent and the next one uniquely identify every syntax fragment in
    // every situation?
    //
    // Additionally, is the next syntax fragment the best piece of information
    // to know, in addition to the parent? Could it be better to know the
    // previous one (in addition to the parent) instead?
    //
    // That would be easier on the parser, and I can't think of any difference
    // as far as uniquely identifying syntax fragments goes. It would be
    // problematic for the evaluator though, as it would need another means to
    // know which syntax fragment to execute next.
    pub next: Option<FragmentId>,
}

impl FragmentAddress {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        if let Some(next) = self.next {
            hasher.update(next.hash.as_bytes());
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxFragment {
    pub payload: SyntaxElement,
    address: FragmentAddress,
}

impl SyntaxFragment {
    pub fn new(payload: SyntaxElement, address: FragmentAddress) -> Self {
        Self { payload, address }
    }

    pub fn next(&self) -> Option<FragmentId> {
        self.address.next
    }

    fn hash(&self) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();

        hasher.update(self.payload.to_string().as_bytes());
        self.address.hash(&mut hasher);

        hasher.finalize()
    }
}

pub type SyntaxToTokens = HashMap<FragmentId, TokenRange>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TokenRange {
    pub start: TokenAddress,
    pub end: TokenAddress,
}

impl TokenRange {
    pub fn one(address: TokenAddress) -> Self {
        Self {
            start: address,
            end: address,
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
