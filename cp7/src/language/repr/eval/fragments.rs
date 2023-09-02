use std::{collections::HashMap, fmt};

use tracing::debug;

use super::value::Value;

#[derive(Debug)]
pub struct Fragments {
    by_id: HashMap<FragmentId, Fragment>,
    by_address: HashMap<FragmentAddress, FragmentId>,
    replacements: HashMap<FragmentId, FragmentId>,
}

impl Fragments {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_address: HashMap::new(),
            replacements: HashMap::new(),
        }
    }

    pub fn insert(&mut self, fragment: Fragment) -> FragmentId {
        let id = FragmentId {
            hash: fragment.hash(),
        };
        let address = fragment.address;

        debug!("insert {}", id.display_short());

        if let Some(existing) = self.by_id.insert(id, fragment.clone()) {
            // A hash collision should be exceedingly unlikely, but I'm not sure
            // quite *how* unlikely. Also, I don't fully trust my code to work
            // perfectly.
            //
            // Let's make sure, just for now, there actually are no hash
            // collisions, okay?
            assert_eq!(existing, fragment);

            // If we replaced an existing entry, then let's end it here.
            // Otherwise, the replacement machinery below will become confused,
            // thinking the fragment replaces itself.
            //
            // Even if that wasn't the case, anything below here is redundant
            // anyway, if there already was an existing entry.
            return id;
        }

        {
            if let Some(existing) = self.by_address.get(&address) {
                // This is a bit too simplistic to detect changes of more than
                // one syntax fragment. It will do for now, but to make this
                // more general, we will eventually have to modify the address
                // by looking at the already detected replacements.

                self.replacements.insert(*existing, id);
            }
        }

        self.by_address.insert(address, id);

        id
    }

    pub fn get(&self, id: FragmentId) -> Fragment {
        // This shouldn't ever panic, as we currently only ever add fragments,
        // never remove them, and only ever create IDs for fragments we add.
        self.by_id.get(&id).cloned().unwrap()
    }

    pub fn take_replacements(&mut self) -> Vec<(FragmentId, FragmentId)> {
        self.replacements.drain().collect()
    }
}

/// Uniquely identifies a syntax fragment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FragmentId {
    hash: blake3::Hash,
}

impl FragmentId {
    pub fn display_short(&self) -> String {
        // We're returning a `String` here, because the allocation doesn't
        // really matter at this point. If we need more efficiency, we can add a
        // custom type with a custom `Display` implementation.
        self.to_string().split_at(4).0.to_string()
    }
}

impl fmt::Display for FragmentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fragment {
    address: FragmentAddress,
    pub payload: FragmentPayload,
}

impl Fragment {
    pub fn new(address: FragmentAddress, payload: FragmentPayload) -> Self {
        Self { address, payload }
    }

    pub fn next(&self) -> Option<FragmentId> {
        self.address.next
    }

    fn hash(&self) -> blake3::Hash {
        let mut hasher = blake3::Hasher::new();

        self.address.hash(&mut hasher);
        self.payload.hash(&mut hasher);

        hasher.finalize()
    }
}

/// Uniquely identifies the location of a syntax fragment in the code
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FragmentAddress {
    pub parent: Option<FragmentId>,
    pub next: Option<FragmentId>,
}

impl FragmentAddress {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        if let Some(parent) = self.parent {
            hasher.update(parent.hash.as_bytes());
        }
        if let Some(next) = self.next {
            hasher.update(next.hash.as_bytes());
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FragmentPayload {
    Value(Value),
    Word(String),
}

impl FragmentPayload {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            FragmentPayload::Value(Value::Block { start }) => {
                hasher.update(b"block");
                if let Some(start) = start {
                    hasher.update(start.hash.as_bytes());
                }
            }
            FragmentPayload::Value(Value::Number(number)) => {
                hasher.update(b"number");
                hasher.update(&number.to_le_bytes());
            }
            FragmentPayload::Value(Value::Symbol(symbol)) => {
                hasher.update(b"symbol");
                hasher.update(symbol.as_bytes());
            }
            FragmentPayload::Word(word) => {
                hasher.update(b"word");
                hasher.update(word.as_bytes());
            }
        }
    }
}
