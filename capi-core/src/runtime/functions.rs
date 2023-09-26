use std::collections::BTreeMap;

use crate::{
    intrinsics,
    repr::eval::{
        fragments::{FragmentId, FragmentPayload, Fragments},
        value::{self, ValueKind},
    },
};

use super::{data_stack::DataStackResult, evaluator::Evaluator};

#[derive(Debug)]
pub struct Functions<C> {
    inner: BTreeMap<String, Function<C>>,
}

impl<C> Functions<C> {
    pub fn new() -> Self {
        let mut inner = BTreeMap::new();

        let intrinsics = [
            ("+", intrinsics::add as IntrinsicFunction),
            ("clone", intrinsics::clone),
            ("eval", intrinsics::eval),
            ("fn", intrinsics::fn_),
            ("nop", intrinsics::nop),
            ("over", intrinsics::over),
            ("swap", intrinsics::swap),
        ];

        for (name, intrinsic) in intrinsics {
            inner.insert(name.to_string(), Function::Intrinsic(intrinsic));
        }

        Self { inner }
    }

    pub fn register_platform(
        &mut self,
        functions: impl IntoIterator<Item = (&'static str, PlatformFunction<C>)>,
    ) {
        for (name, function) in functions {
            self.inner.insert(name.into(), Function::Platform(function));
        }
    }

    pub fn define(&mut self, name: FunctionName, body: value::Block) {
        let function = Function::UserDefined(UserDefinedFunction {
            name: name.clone(),
            body,
        });
        self.inner.insert(name.value, function);
    }

    pub fn resolve(&self, name: &str) -> Result<Function<C>, ResolveError>
    where
        C: Clone,
    {
        self.inner
            .get(name)
            .cloned()
            .ok_or(ResolveError { name: name.into() })
    }

    pub fn replace(
        &mut self,
        old: FragmentId,
        new: FragmentId,
        fragments: &Fragments,
    ) {
        let mut renames = Vec::new();

        for (old_name, function) in self.inner.iter_mut() {
            if let Function::UserDefined(UserDefinedFunction { name, body }) =
                function
            {
                if name.fragment == Some(old) {
                    let fragment = fragments.get(new);
                    let FragmentPayload::Value(ValueKind::Symbol(new_name)) =
                        &fragment.payload
                    else {
                        // If the new fragment is not a symbol, then it's not
                        // supposed to be a function name. Not sure if we can
                        // handle this any smarter.
                        continue;
                    };

                    name.value = new_name.clone();
                    name.fragment = Some(new);

                    renames.push((old_name.clone(), new_name.clone()));
                }
                if body.start == old {
                    body.start = new;
                }
            }
        }

        for (old, new) in renames {
            let function = self.inner.remove(&old).unwrap();
            self.inner.insert(new, function);
        }
    }
}

impl<C> Default for Functions<C> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub enum Function<C> {
    Intrinsic(IntrinsicFunction),
    Platform(PlatformFunction<C>),
    UserDefined(UserDefinedFunction),
}

pub type IntrinsicFunction = fn(&mut Evaluator) -> DataStackResult<()>;
pub type PlatformFunction<C> =
    fn(&mut Evaluator, &mut C) -> DataStackResult<()>;

#[derive(Clone, Debug)]
pub struct UserDefinedFunction {
    pub name: FunctionName,
    pub body: value::Block,
}

#[derive(Clone, Debug)]
pub struct FunctionName {
    pub value: String,
    pub fragment: Option<FragmentId>,
}

#[derive(Debug, thiserror::Error)]
#[error("Error resolving function `{name}`")]
pub struct ResolveError {
    pub name: String,
}
