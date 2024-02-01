use std::{collections::BTreeMap, fmt};

use crate::{
    intrinsics,
    pipeline::{Function, Module},
    repr::eval::{
        fragments::{FragmentId, Fragments},
        value::Value,
    },
    runtime::namespaces::PlatformFunction,
};

use super::{IntrinsicFunction, NativeFunction};

#[derive(Debug)]
pub struct Namespace<C> {
    native_functions: BTreeMap<String, NativeFunction<C>>,
    global_module: Module,
}

impl<C> Namespace<C> {
    pub fn new(global_module: Module) -> Self {
        let mut native_functions = BTreeMap::new();

        for (intrinsic, name) in intrinsics::all() {
            native_functions
                .insert(name.to_string(), NativeFunction::Intrinsic(intrinsic));
        }

        Self {
            native_functions,
            global_module,
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

    pub fn global_module(&mut self) -> &mut Module {
        &mut self.global_module
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
            .global_module
            .functions
            .0
            .get(name)
            .map(|user_defined| {
                ItemInModule::UserDefinedFunction(user_defined)
            });
        let binding = self
            .global_module
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

        self.global_module.functions.replace(old, new, fragments);
        self.global_module.tests.replace(old, new, fragments);
    }

    pub fn into_module(self) -> Module {
        self.global_module
    }
}

#[derive(Clone)]
pub enum ItemInModule<'r, C> {
    Binding(Value),
    IntrinsicFunction(IntrinsicFunction),
    PlatformFunction(&'r PlatformFunction<C>),
    UserDefinedFunction(&'r Function),
}

impl<C> fmt::Debug for ItemInModule<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binding(value) => {
                f.debug_tuple("Binding").field(value).finish()
            }
            Self::IntrinsicFunction(intrinsic_function) => f
                .debug_tuple("IntrinsicFunction")
                .field(intrinsic_function)
                .finish(),
            Self::PlatformFunction(platform_function) => f
                .debug_tuple("PlatformFunction")
                .field(platform_function)
                .finish(),
            Self::UserDefinedFunction(user_defined_function) => f
                .debug_tuple("UserDefinedFunction")
                .field(user_defined_function)
                .finish(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Error resolving function `{name}`")]
pub struct ResolveError {
    pub name: String,
}
