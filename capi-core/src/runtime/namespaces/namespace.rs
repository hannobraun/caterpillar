use std::{collections::BTreeMap, fmt};

use crate::{
    pipeline::{Function, Module},
    platform::{core::CorePlatform, BuiltinFn, Platform},
    repr::eval::value::Value,
};

pub struct Namespace<P: Platform> {
    native_functions: BTreeMap<String, Builtin<P>>,
    global_module: Module,
}

impl<P: Platform> Namespace<P> {
    pub fn new(global_module: Module) -> Self {
        let mut native_functions = BTreeMap::new();

        for (intrinsic, name) in CorePlatform::builtin_fns() {
            native_functions.insert(name.to_string(), Builtin::Core(intrinsic));
        }

        Self {
            native_functions,
            global_module,
        }
    }

    pub fn register_platform(
        &mut self,
        functions: impl IntoIterator<Item = (BuiltinFn<P>, &'static str)>,
    ) {
        for (function, name) in functions {
            self.native_functions
                .insert(name.into(), Builtin::Platform(function));
        }
    }

    pub fn global_module(&mut self) -> &mut Module {
        &mut self.global_module
    }

    pub fn resolve(&self, name: &str) -> Result<ItemInModule<P>, ResolveError> {
        let native_function =
            self.native_functions.get(name).map(|native| match native {
                Builtin::Core(function) => {
                    ItemInModule::IntrinsicFunction(*function)
                }
                Builtin::Platform(function) => {
                    ItemInModule::PlatformFunction(function)
                }
            });
        let user_defined_function =
            self.global_module.functions.get(name).map(|user_defined| {
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

    pub fn into_module(self) -> Module {
        self.global_module
    }
}

#[derive(Clone)]
pub enum ItemInModule<'r, P: Platform + 'r> {
    Binding(Value),
    IntrinsicFunction(BuiltinFn<CorePlatform>),
    PlatformFunction(&'r BuiltinFn<P>),
    UserDefinedFunction(&'r Function),
}

impl<P: Platform> fmt::Debug for ItemInModule<'_, P> {
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

#[derive(Debug)]
pub enum Builtin<P: Platform> {
    Core(BuiltinFn<CorePlatform>),
    Platform(BuiltinFn<P>),
}

#[derive(Debug, thiserror::Error)]
#[error("Error resolving function `{name}`")]
pub struct ResolveError {
    pub name: String,
}
