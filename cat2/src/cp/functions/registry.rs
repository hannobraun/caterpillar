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

        let mut by_name = self
            .inner
            .iter()
            .filter(|f| f.name == name)
            .collect::<Vec<_>>();
        by_name.sort_by_key(|f| f.args.len());
        by_name.reverse();

        let mut matched_values = Vec::new();
        loop {
            // This is a prime candidate for using `Vec::drain_filter`, once
            // that is stable.
            let mut with_matching_signature_len = Vec::new();
            let mut i = 0;
            while i < by_name.len() {
                if by_name[i].args.len() == matched_values.len() {
                    with_matching_signature_len.push(by_name.remove(i));
                } else {
                    i += 1;
                }
            }

            let mut prime_candidate = with_matching_signature_len.pop();
            for candidate in with_matching_signature_len.drain(..) {
                let type_of_last_value = matched_values.last().map(Value::ty);
                let type_of_last_arg = candidate.args.inner.last().map(Arg::ty);

                let is_correct_type = type_of_last_value == type_of_last_arg;

                if is_correct_type {
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
}
