mod id;

pub use self::id::FragmentId;

use std::collections::HashMap;

use super::value::Value;

#[derive(Debug)]
pub struct Fragments {
    by_id: HashMap<FragmentId, Fragment>,
    by_address: HashMap<FragmentAddress, FragmentId>,
    by_next_fragment: HashMap<FragmentId, FragmentId>,
    replacements: Replacements,
}

impl Fragments {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_address: HashMap::new(),
            by_next_fragment: HashMap::new(),
            replacements: Replacements {
                inner: HashMap::new(),
            },
        }
    }

    pub fn insert(&mut self, fragment: Fragment) -> FragmentId {
        let id = FragmentId {
            hash: fragment.hash(),
        };
        let address = fragment.address;

        if let Some(existing) = self.by_id.insert(id, fragment.clone()) {
            // A hash collision should be exceedingly unlikely, but I'm not sure
            // quite *how* unlikely. Also, I don't fully trust my code to work
            // perfectly.
            //
            // Let's make sure, just for now, there actually are no hash
            // collisions, okay?
            assert_eq!(existing, fragment);
        } else {
            let id = id.display_short();
            let payload = fragment.payload.display_short();
            let address = address.display_short();

            eprintln!("insert {id} ({payload}) at {address}");
        }

        if let Some(next) = address.next {
            self.by_next_fragment.insert(next, id);
        }

        {
            if let Some(existing) = self.by_address.get(&address).copied() {
                // This is a bit too simplistic to detect changes of more than
                // one syntax fragment. It will do for now, but to make this
                // more general, we will eventually have to modify the address
                // by looking at the already detected replacements.

                if existing != id {
                    self.replacements.inner.insert(existing, id);

                    let existing = existing.display_short();
                    let id = id.display_short();
                    eprintln!("Replace {existing} with {id}");
                }
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
        self.replacements.inner.drain().collect()
    }
}

#[derive(Debug)]
struct Replacements {
    inner: HashMap<FragmentId, FragmentId>,
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
    pub fn display_short(&self) -> String {
        format!(
            "{{ parent: {:?}, next: {:?} }}",
            self.parent.map(|id| id.display_short()),
            self.next.map(|id| id.display_short())
        )
    }

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
    pub fn display_short(&self) -> String {
        match self {
            FragmentPayload::Value(value) => {
                let value = value.display_short();
                format!("value `{value}`")
            }
            FragmentPayload::Word(word) => format!("word `{word}`"),
        }
    }

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
