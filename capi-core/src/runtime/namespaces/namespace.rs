use std::collections::BTreeMap;

use crate::{
    intrinsics,
    repr::eval::{
        fragments::{FragmentId, FragmentPayload, Fragments},
        value::ValuePayload,
    },
    value::Value,
    PlatformFunction,
};

use super::{
    functions::Functions, IntrinsicFunction, NativeFunction,
    UserDefinedFunction, UserDefinedItems,
};

#[derive(Debug)]
pub struct Namespace<C> {
    bindings: BTreeMap<String, Value>,
    native_functions: BTreeMap<String, NativeFunction<C>>,
    user_defined_functions: Functions,
    tests: Functions,
}

impl<C> Namespace<C> {
    pub fn new() -> Self {
        let mut native_functions = BTreeMap::new();

        for (intrinsic, name) in intrinsics::all() {
            native_functions
                .insert(name.to_string(), NativeFunction::Intrinsic(intrinsic));
        }

        Self {
            bindings: BTreeMap::new(),
            native_functions,
            user_defined_functions: Functions::new(),
            tests: Functions::new(),
        }
    }

    pub fn register_platform(
        &mut self,
        functions: impl IntoIterator<Item = (PlatformFunction<C>, &'static str)>,
    ) {
        for (function, name) in functions {
            self.native_functions
                .insert(name.into(), NativeFunction::Platform(function));
        }
    }

    pub fn user_defined(&mut self) -> UserDefinedItems {
        UserDefinedItems {
            bindings: &mut self.bindings,
            functions: &mut self.user_defined_functions.0,
            tests: &mut self.tests.0,
        }
    }

    pub fn resolve(&self, name: &str) -> Result<ItemInModule<C>, ResolveError> {
        let native_function =
            self.native_functions.get(name).map(|native| match native {
                NativeFunction::Intrinsic(function) => {
                    ItemInModule::IntrinsicFunction(*function)
                }
                NativeFunction::Platform(function) => {
                    ItemInModule::PlatformFunction(function)
                }
            });
        let user_defined_function =
            self.user_defined_functions.0.get(name).map(|user_defined| {
                ItemInModule::UserDefinedFunction(user_defined)
            });
        let binding = self
            .bindings
            .get(name)
            .map(|binding| ItemInModule::Binding(binding.clone()));

        native_function
            .or(user_defined_function)
            .or(binding)
            .ok_or(ResolveError { name: name.into() })
    }

    pub fn replace(
        &mut self,
        old: FragmentId,
        new: FragmentId,
        fragments: &Fragments,
    ) {
        // This function has at least two limitations:
        //
        // - It only detects *renames*. It does not detect *removals*. Maybe we
        //   need to take an `Option<FragmentId>` as the `new` argument, and
        //   handle that here accordingly.
        // - It does not detect renamed tests.

        let mut renames = Vec::new();

        for (old_name, UserDefinedFunction { name, body, .. }) in
            self.user_defined_functions.0.iter_mut()
        {
            if name.fragment == Some(old) {
                let fragment = fragments.get(new);
                let FragmentPayload::Value(ValuePayload::Symbol(new_name)) =
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
            let function = self.user_defined_functions.0.remove(&old).expect(
                "Found `old` in the map; expecting it to still be there",
            );
            self.user_defined_functions.0.insert(new, function);
        }
    }
}

impl<C> Default for Namespace<C> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub enum ItemInModule<'r, C> {
    Binding(Value),
    IntrinsicFunction(IntrinsicFunction),
    PlatformFunction(&'r PlatformFunction<C>),
    UserDefinedFunction(&'r UserDefinedFunction),
}

#[derive(Debug, thiserror::Error)]
#[error("Error resolving function `{name}`")]
pub struct ResolveError {
    pub name: String,
}
