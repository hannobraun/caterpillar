use std::collections::BTreeMap;

use super::{tokenize, tokenizer::Tokens};

pub struct Functions {
    inner: BTreeMap<String, Function>,
}

impl Functions {
    pub fn new() -> Self {
        let inner = BTreeMap::new();
        let mut self_ = Self { inner };

        // Eventually, we'll store the source code in a persistent way. But for
        // now, we'll just define default code on startup, as a starting point
        // for the user to modify.
        self_.define(
            String::from("cell_is_born"),
            Function {
                tokens: tokenize("clone 2 = swap 3 = or"),
            },
        );
        self_.define(
            String::from("cell_survives"),
            Function {
                tokens: tokenize("clone 2 = swap 4 = or"),
            },
        );

        self_
    }

    pub fn define(&mut self, name: impl Into<String>, function: Function) {
        self.inner.insert(name.into(), function);
    }

    pub fn get(&self, name: &str) -> &Tokens {
        &self
            .inner
            .get(name)
            .unwrap_or_else(|| panic!("Function {name} not defined"))
            .tokens
    }

    pub fn get_mut(&mut self, name: &str) -> &mut Tokens {
        &mut self
            .inner
            .get_mut(name)
            .unwrap_or_else(|| panic!("Function {name} not defined"))
            .tokens
    }
}

pub struct Function {
    pub tokens: Tokens,
}
