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

    #[cfg(test)]
    pub fn resolve(
        &self,
        _: impl Into<String>,
        values: impl IntoIterator<Item = crate::cp::Value>,
    ) -> Option<&Function> {
        let values = values.into_iter().collect::<Vec<_>>();

        // We'll use this variable to store our current best candidate in. So
        // far we haven't looked at any, so it's empty.
        let mut prime_candidate: Option<&Function> = None;

        'outer: for next_candidate in &self.inner {
            // Let's look at some criteria that would disqualify the function
            // from being a match.
            for (arg, value) in next_candidate.args.inner.iter().zip(&values) {
                if arg.ty() != value.ty() {
                    continue 'outer;
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

    pub fn get(
        &self,
        name: impl Into<String>,
        args: impl Into<Args>,
    ) -> Option<&Function> {
        let name = name.into();
        let args = args.into();

        self.inner
            .iter()
            .find(|function| function.name == name && function.args == args)
    }

    pub fn get_mut(
        &mut self,
        name: impl Into<String>,
        args: impl Into<Args>,
    ) -> Option<&mut Function> {
        let name = name.into();
        let args = args.into();

        self.inner
            .iter_mut()
            .find(|function| function.name == name && function.args == args)
    }
}

#[cfg(test)]
mod tests {
    use crate::cp::{functions::Args, Arg, Type, Value};

    use super::Registry;

    #[test]
    fn resolve_matching_type() {
        let mut registry = Registry::new();
        registry.define("name", [Arg::Type(Type::U8)], "");
        registry.define("name", [Arg::Type(Type::Bool)], "");

        let f = registry.resolve("name", [Value::U8(0)]).unwrap();

        assert_eq!(f.name, "name");
        assert_eq!(f.args, Args::from([Arg::Type(Type::U8)]));
    }

    #[test]
    fn resolve_simplest_signature() {
        let mut registry = Registry::new();
        registry.define("name", [], "");
        registry.define("name", [Arg::Type(Type::Bool)], "");

        let f = registry.resolve("name", [Value::Bool(true)]).unwrap();

        assert_eq!(f.name, "name");
        assert_eq!(f.args, Args::from([]));
    }

    #[test]
    fn resolve_prefer_value() {
        let mut registry = Registry::new();
        registry.define("name", [Arg::Value(Value::Bool(true))], "");
        registry.define("name", [Arg::Type(Type::Bool)], "");

        let f = registry.resolve("name", [Value::Bool(true)]).unwrap();

        assert_eq!(f.name, "name");
        assert_eq!(f.args, Args::from([Arg::Value(Value::Bool(true))]));
    }
}
