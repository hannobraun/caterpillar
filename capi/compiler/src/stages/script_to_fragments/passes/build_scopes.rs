use std::collections::BTreeSet;

use crate::repr::syntax::Expression;

pub fn build_scopes(args: Vec<String>, body: &[Expression]) -> Scopes {
    let mut bindings = Bindings {
        inner: args.into_iter().collect(),
    };

    bindings.process_block(body);

    Scopes { inner: bindings }
}

pub struct Scopes {
    inner: Bindings,
}

impl Scopes {
    pub fn binding_resolves(&self, name: &str) -> bool {
        self.inner.inner.contains(name)
    }
}

struct Bindings {
    inner: BTreeSet<String>,
}

impl Bindings {
    pub fn process_block(&mut self, block: &[Expression]) {
        for expression in block {
            if let Expression::Binding { names } = expression {
                for name in names.iter().cloned().rev() {
                    // Inserting bindings unconditionally like this does mean
                    // that bindings can overwrite previously defined bindings.
                    // This is undesirable, but it'll do for now.
                    self.inner.insert(name);
                }
            }
            if let Expression::Block { expressions } = expression {
                self.process_block(expressions);
            }
        }
    }
}
