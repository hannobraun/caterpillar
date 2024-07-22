use std::collections::BTreeSet;

use crate::repr::syntax::Expression;

pub fn process_function(args: Vec<String>, body: &[Expression]) -> Scopes {
    let mut scopes = Scopes {
        stack: vec![Bindings {
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
                scopes.stack.last_mut().unwrap().inner.insert(name);
            }
        }
        if let Expression::Block { expressions } = expression {
            scopes.stack.push(Bindings {
                inner: BTreeSet::new(),
            });
            process_block(expressions, scopes);
        }
    }
}

pub struct Scopes {
    stack: Vec<Bindings>,
}

impl Scopes {
    pub fn resolve_binding(&self, name: &str) -> Option<BindingResolved> {
        let mut scopes = self.stack.iter().rev();

        if let Some(scope) = scopes.next() {
            if scope.inner.contains(name) {
                return Some(BindingResolved::InScope);
            }
        }

        for scope in scopes {
            if scope.inner.contains(name) {
                return Some(BindingResolved::InEnvironment);
            }
        }

        None
    }
}

pub enum BindingResolved {
    InScope,
    InEnvironment,
}

struct Bindings {
    inner: BTreeSet<String>,
}
