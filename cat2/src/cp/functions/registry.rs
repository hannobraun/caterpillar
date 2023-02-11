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
        name: impl Into<String>,
        values: impl IntoIterator<Item = crate::cp::Value>,
    ) -> Option<&Function> {
        use crate::cp::Value;

        use super::Arg;

        let name = name.into();
        let mut values = values.into_iter();

        // Find all functions with the correct name, and sort them by the number
        // of their arguments. We'll `.pop()` functions from the end of the
        // list, so we'll be looking at those with the fewest arguments first.
        let mut candidates = self
            .inner
            .iter()
            .filter(|f| f.name == name)
            .collect::<Vec<_>>();
        candidates.sort_by_key(|f| f.args.len());
        candidates.reverse();

        // Now we'll be looking at the candidates and match them against the
        // values to find the best one. We expect the `values` iterator to
        // provide the top value on the stack first. We'll move down the stack
        // until we find a match or run out of candidates.
        //
        // We store the values we've already matches against in this `Vec`.
        // Since we haven't yet matched against any values, it starts out empty.
        let mut matched_values = Vec::new();

        loop {
            // This is a prime candidate for using `Vec::drain_filter`, once
            // that is stable.
            let mut with_matching_signature_len = Vec::new();
            let mut i = 0;
            while i < candidates.len() {
                if candidates[i].args.len() == matched_values.len() {
                    with_matching_signature_len.push(candidates.remove(i));
                } else {
                    i += 1;
                }
            }

            let mut prime_candidate = with_matching_signature_len.pop();
            for candidate in with_matching_signature_len.drain(..) {
                let last_matched_value = matched_values.last();
                let last_arg_of_candidate = candidate.args.last();

                let type_of_last_value = last_matched_value.map(Value::ty);
                let type_of_last_arg = last_arg_of_candidate.map(Arg::ty);

                let is_correct_type = type_of_last_value == type_of_last_arg;
                let is_strong_enough_match = prime_candidate
                    .and_then(|prime_candidate| prime_candidate.args.last())
                    .map(Arg::is_type)
                    .unwrap_or(true)
                    || candidate
                        .args
                        .last()
                        .map(Arg::is_value)
                        .unwrap_or(false);

                if is_correct_type && is_strong_enough_match {
                    prime_candidate = Some(candidate);
                }
            }

            if prime_candidate.is_some() {
                return prime_candidate;
            }

            let next_value = values.next()?;
            matched_values.push(next_value);
        }
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
        registry.define("name", [Arg::Type(Type::Bool)], "");
        registry.define("name", [Arg::Type(Type::U8)], "");

        let f = registry.resolve("name", [Value::U8(0)]).unwrap();

        assert_eq!(f.name, "name");
        assert_eq!(f.args, Args::from([Arg::Type(Type::U8)]));
    }

    #[test]
    fn resolve_simplest_signature() {
        let mut registry = Registry::new();
        registry.define("name", [Arg::Type(Type::Bool)], "");
        registry.define("name", [], "");

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
