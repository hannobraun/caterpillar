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
    native: BTreeMap<String, Function<C>>,
    user_defined: BTreeMap<String, UserDefinedFunction>,
}

impl<C> Functions<C> {
    pub fn new() -> Self {
        let mut native = BTreeMap::new();

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
            native.insert(name.to_string(), Function::Intrinsic(intrinsic));
        }

        Self {
            native,
            user_defined: BTreeMap::new(),
        }
    }

    pub fn register_platform(
        &mut self,
        functions: impl IntoIterator<Item = (&'static str, PlatformFunction<C>)>,
    ) {
        for (name, function) in functions {
            self.native
                .insert(name.into(), Function::Platform(function));
        }
    }

    pub fn define(&mut self, name: FunctionName, body: value::Block) {
        let function = UserDefinedFunction {
            name: name.clone(),
            body,
        };
        self.user_defined.insert(name.value, function);
    }

    pub fn resolve(&self, name: &str) -> Result<Function<C>, ResolveError>
    where
        C: Clone,
    {
        let native = self.native.get(name).cloned();
        let user_defined = self
            .user_defined
            .get(name)
            .map(|function| Function::UserDefined(function.clone()));

        native
            .or(user_defined)
            .ok_or(ResolveError { name: name.into() })
    }

    pub fn replace(
        &mut self,
        old: FragmentId,
        new: FragmentId,
        fragments: &Fragments,
    ) {
        let mut renames = Vec::new();

        for (old_name, UserDefinedFunction { name, body }) in
            self.user_defined.iter_mut()
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

        for (old, new) in renames {
            let function = self.user_defined.remove(&old).unwrap();
            self.user_defined.insert(new, function);
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
