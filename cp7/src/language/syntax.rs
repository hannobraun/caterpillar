use std::{collections::HashMap, fmt};

use super::{tokens::TokenAddress, value::Value};

#[derive(Debug)]
pub struct Syntax {
    by_id: HashMap<FragmentId, SyntaxFragment>,
    by_address: HashMap<FragmentAddress, FragmentId>,
    replacements: HashMap<blake3::Hash, blake3::Hash>,
}

impl Syntax {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_address: HashMap::new(),
            replacements: HashMap::new(),
        }
    }

    pub fn add(&mut self, fragment: SyntaxFragment) -> FragmentId {
        let id = FragmentId {
            hash: fragment.hash(),
        };
        let address = fragment.address;

        // A hash collision should be exceedingly unlikely, but I'm not sure
        // quite *how* unlikely. Also, I don't fully trust my code to work
        // perfectly.
        //
        // Let's make sure, just for now, there actually are no hash collisions,
        // okay?
        if let Some(existing) = self.by_id.get(&id) {
            assert_eq!(existing, &fragment);
        }

        self.by_id.insert(id, fragment);

        if let Some(existing) = self.by_address.get(&address) {
            // This is a bit too simplistic to detect changes of more than one
            // syntax fragment. It will do for now, but to make this more
            // general, we will eventually have to modify the address by looking
            // at the already detected replacements.

            self.replacements.insert(existing.hash, id.hash);
        }

        self.by_address.insert(address, id);

        id
    }

    pub fn get(&self, handle: FragmentId) -> SyntaxFragment {
        // This shouldn't ever panic, as we currently only ever add fragments,
        // never remove them, and only ever create handles for fragments we add.
        self.by_id.get(&handle).cloned().unwrap()
    }

    pub fn take_replacements(&mut self) -> Vec<(FragmentId, FragmentId)> {
        self.replacements
            .drain()
            .map(|(old, new)| {
                (FragmentId { hash: old }, FragmentId { hash: new })
            })
            .collect()
    }
}

/// Uniquely identifies a syntax fragment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FragmentId {
    pub hash: blake3::Hash,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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
