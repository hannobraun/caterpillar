use std::collections::BTreeMap;

use super::{tokenize, tokenizer::Tokens};

pub struct Functions {
    inner: BTreeMap<String, Tokens>,
}

impl Functions {
    pub fn new() -> Self {
        let mut inner = BTreeMap::new();

        // Eventually, we'll store the source code in a persistent way. But for
        // now, we'll just define default code on startup, as a starting point
        // for the user to modify.
        inner.insert(
            String::from("cell_is_born"),
            tokenize("clone 2 = swap 3 = or"),
        );
        inner.insert(
            String::from("cell_survives"),
            tokenize("clone 2 = swap 4 = or"),
        );

        Self { inner }
    }

    pub fn get(&self, name: &str) -> &Tokens {
        self.inner
            .get(name)
            .unwrap_or_else(|| panic!("Function {name} not defined"))
    }

    pub fn get_mut(&mut self, name: &str) -> &mut Tokens {
        self.inner
            .get_mut(name)
            .unwrap_or_else(|| panic!("Function {name} not defined"))
    }
}
