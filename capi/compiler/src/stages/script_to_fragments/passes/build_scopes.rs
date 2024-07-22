use std::collections::BTreeSet;

use crate::repr::syntax::Expression;

pub fn process_function(args: Vec<String>, body: &[Expression]) -> Scopes {
    let mut scopes = Scopes {
        inner: vec![Bindings {
            inner: args.into_iter().collect(),
        }],
    };

    process_block(body, &mut scopes);

    scopes
}

fn process_block(body: &[Expression], scopes: &mut Scopes) {
    for expression in body {
        if let Expression::Binding { names } = expression {
            for name in names.iter().cloned().rev() {
                // Inserting bindings unconditionally like this does mean
                // that bindings can overwrite previously defined bindings.
                // This is undesirable, but it'll do for now.
                scopes.inner.last_mut().unwrap().inner.insert(name);
            }
        }
        if let Expression::Block { expressions } = expression {
            process_block(expressions, scopes);
        }
    }
}

pub struct Scopes {
    inner: Vec<Bindings>,
}

impl Scopes {
    pub fn binding_resolves(&self, name: &str) -> bool {
        self.inner.last().unwrap().inner.contains(name)
    }
}

struct Bindings {
    inner: BTreeSet<String>,
}
