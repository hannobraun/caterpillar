use std::collections::BTreeMap;

use crate::{
    intrinsics,
    module::function::Function,
    repr::eval::fragments::{FragmentId, Fragments},
    value::Value,
    PlatformFunction,
};

use super::{IntrinsicFunction, NativeFunction, UserDefinedItems};

#[derive(Debug)]
pub struct Namespace<C> {
    native_functions: BTreeMap<String, NativeFunction<C>>,
    user_defined_items: UserDefinedItems,
}

impl<C> Namespace<C> {
    pub fn new() -> Self {
        let mut native_functions = BTreeMap::new();

        for (intrinsic, name) in intrinsics::all() {
            native_functions
                .insert(name.to_string(), NativeFunction::Intrinsic(intrinsic));
        }

        Self {
            native_functions,
            user_defined_items: UserDefinedItems::default(),
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

    pub fn user_defined(&mut self) -> &mut UserDefinedItems {
        &mut self.user_defined_items
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
        let user_defined_function = self
            .user_defined_items
            .functions
            .0
            .get(name)
            .map(|user_defined| {
                ItemInModule::UserDefinedFunction(user_defined)
            });
        let binding = self
            .user_defined_items
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
        // This function only detects *renames*. It does not detect *removals*.
        // Maybe we need to take an `Option<FragmentId>` as the `new` argument,
        // and handle that here accordingly.

        self.user_defined_items
            .functions
            .replace(old, new, fragments);
        self.user_defined_items.tests.replace(old, new, fragments);
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
    UserDefinedFunction(&'r Function),
}

#[derive(Debug, thiserror::Error)]
#[error("Error resolving function `{name}`")]
pub struct ResolveError {
    pub name: String,
}
