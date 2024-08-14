use capi_process::Value;

use crate::syntax::Pattern;

use super::{
    Branch, Fragment, FragmentExpression, FragmentId, FragmentPayload,
    Function, Parameters,
};

/// # Extension trait for types that provide a hash
///
/// The purpose of this extension trait is to centralize all hash-related code,
/// and also to provide a central place where guidelines for such code can be
/// documented.
///
/// ## Rules for Implementations
///
/// With a good hash function, it should be exceedingly unlikely to get hash
/// collisions for different values. It is, however, still possible to make hash
/// collisions quite likely, through sloppy use of the hash function. The
/// following rules are designed to prevent that.
///
/// **1. Implementations must only access fields via destructuring.**
///
/// This happens automatically when implementing this trait for an enum, but not
/// when implementing it for a struct. `..` in patterns is forbidden in all
/// implementations.
///
/// By following this rule, we make sure that adding a new field to a struct or
/// enum results in a compiler error in the respective implementation. By
/// ignoring it, we risk forgetting to update the implementation, which could
/// lead to hash collisions between different values that only differ in fields
/// that are not included in the hash.
///
/// **2. Hashing an enum variant must include the variant's name.**
///
/// Different variants of an enum might have similar fields. To prevent hash
/// collisions between different variants that happen to have equal values in
/// their fields, we need to feed different data to the respective hashers.
///
/// Any unique string will do, but by convention, we use the name of the enum
/// variant.
pub(super) trait FragmentHash {
    fn hash(&self, hasher: &mut blake3::Hasher);
}

impl FragmentHash for Branch {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        let Self { parameters, start } = self;

        parameters.hash(hasher);
        start.hash(hasher);
    }
}

impl FragmentHash for Fragment {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        let Self { parent, payload } = self;

        if let Some(parent) = parent.as_ref() {
            parent.hash(hasher);
        }
        payload.hash(hasher);
    }
}

impl FragmentHash for FragmentExpression {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            Self::BindingDefinitions { names } => {
                hasher.update(b"BindingDefinitions");

                for name in names {
                    hasher.update(name.as_bytes());
                }
            }
            Self::Block { start, environment } => {
                hasher.update(b"Block");
                start.hash(hasher);
                for binding in environment {
                    hasher.update(binding.as_bytes());
                }
            }
            Self::Comment { text } => {
                hasher.update(b"Comment");
                hasher.update(text.as_bytes());
            }
            Self::ResolvedBinding { name } => {
                hasher.update(b"ResolvedBinding");
                hasher.update(name.as_bytes());
            }
            Self::ResolvedBuiltinFunction { name } => {
                hasher.update(b"ResolvedBuiltinFunction");
                hasher.update(name.as_bytes());
            }
            Self::ResolvedFunction { name, is_tail_call } => {
                hasher.update(b"ResolvedFunction");
                hasher.update(name.as_bytes());
                hasher.update(&[(*is_tail_call).into()]);
            }
            Self::ResolvedHostFunction { name } => {
                hasher.update(b"ResolvedHostFunction");
                hasher.update(name.as_bytes());
            }
            Self::UnresolvedIdentifier { name } => {
                hasher.update(b"UnresolvedIdentifier");
                hasher.update(name.as_bytes());
            }
            Self::Value(value) => {
                let Value(value) = value;

                hasher.update(b"Value");
                hasher.update(value);
            }
        }
    }
}

impl FragmentHash for FragmentId {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        let Self { hash } = self;

        hasher.update(hash.as_bytes());
    }
}

impl FragmentHash for FragmentPayload {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            Self::Function { function, next } => {
                hasher.update(b"Function");
                function.hash(hasher);
                next.hash(hasher);
            }
            Self::Expression { expression, next } => {
                hasher.update(b"Expression");
                expression.hash(hasher);
                next.hash(hasher);
            }
            Self::Terminator => {
                hasher.update(b"Terminator");
            }
        }
    }
}

impl FragmentHash for Function {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        let Self { name, branches } = self;

        if let Some(name) = name {
            hasher.update(name.as_bytes());
        }
        for branch in branches {
            branch.hash(hasher);
        }
    }
}

impl FragmentHash for Parameters {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        let Self { inner } = self;

        for argument in inner {
            match argument {
                Pattern::Identifier { name } => {
                    hasher.update(b"Identifier");
                    hasher.update(name.as_bytes());
                }
                Pattern::Literal { value } => {
                    hasher.update(b"Literal");
                    hasher.update(&value.0);
                }
            }
        }
    }
}
