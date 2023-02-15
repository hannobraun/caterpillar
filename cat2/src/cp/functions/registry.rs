use crate::cp::Value;

use super::{Args, Function};

pub struct Registry {
    inner: Vec<Function>,
}

impl Registry {
    pub fn new() -> Self {
        let inner = Vec::new();
        Self { inner }
    }

    pub fn define(
        &mut self,
        name: impl Into<String>,
        args: impl Into<Args>,
        body: &str,
    ) {
        self.inner.push(Function::new(name, args, body));
    }

    pub fn resolve(
        &self,
        name: &str,
        values: impl IntoIterator<Item = Value>,
    ) -> Option<&Function> {
        let values = values.into_iter().collect::<Vec<_>>();

        // We'll use this variable to store our current best candidate in. So
        // far we haven't looked at any, so it's empty.
        let mut prime_candidate: Option<&Function> = None;

        'outer: for next_candidate in &self.inner {
            // Let's look at some criteria that would disqualify the function
            // from being a match.
            if next_candidate.name != name {
                continue;
            }
            if next_candidate.args.inner.len() > values.len() {
                continue;
            }
            for (arg, value) in
                next_candidate.args.inner.iter().rev().zip(&values)
            {
                if arg.ty() != value.ty() {
                    continue 'outer;
                }

                if let Some(next_value) = arg.value() {
                    if next_value != value {
                        continue 'outer;
                    }
                }
            }

            // The function qualifies! Now let's check if there's anything that
            // makes it worse than the current prime candidate.
            if let Some(prime_candidate) = prime_candidate {
                let args_prime = &prime_candidate.args.inner;
                let args_next = &next_candidate.args.inner;

                if args_prime.len() < args_next.len() {
                    continue 'outer;
                }

                for (a, b) in args_prime.iter().zip(args_next) {
                    if a.is_value() && b.is_type() {
                        continue 'outer;
                    }
                }
            }

            // Nothing found! Replace the current prime candidate.
            prime_candidate = Some(next_candidate);
        }

        prime_candidate
    }
}
