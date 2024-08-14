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
/// **1. Each implementation must start by feeding a unique string.**
///
/// Different implementation might have field types, even identical field names.
/// If two instances of similar implementations happened to feature the same
/// data, this would result in a hash collision.
///
/// By feeding a unique string to the hasher, we prevent this case from
/// occurring. Any unique string string will do, but by convention, we use the
/// name of the type.
///
/// **2. Implementations must only access fields via destructuring.**
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
/// **3. Hashing an enum variant must start with a unique string.**
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
        hasher.update(b"Branch");

        let Self { parameters, start } = self;

        hasher.update(b"parameters");
        parameters.hash(hasher);

        hasher.update(b"start");
        start.hash(hasher);
    }
}

impl FragmentHash for Fragment {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        hasher.update(b"Fragment");

        let Self { parent, payload } = self;

        hasher.update(b"parent");
        if let Some(parent) = parent {
            parent.hash(hasher);
        }

        hasher.update(b"payload");
        payload.hash(hasher);
    }
}

impl FragmentHash for FragmentExpression {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        hasher.update(b"FragmentExpression");

        match self {
            Self::BindingDefinitions { names } => {
                hasher.update(b"BindingDefinitions");

                hasher.update(b"names");
                for name in names {
                    hasher.update(name.as_bytes());
                }
            }
            Self::Block { start, environment } => {
                hasher.update(b"Block");

                hasher.update(b"start");
                start.hash(hasher);

                hasher.update(b"environment");
                for binding in environment {
                    hasher.update(binding.as_bytes());
                }
            }
            Self::Comment { text } => {
                hasher.update(b"Comment");

                hasher.update(b"text");
                hasher.update(text.as_bytes());
            }
            Self::ResolvedBinding { name } => {
                hasher.update(b"ResolvedBinding");

                hasher.update(b"name");
                hasher.update(name.as_bytes());
            }
            Self::ResolvedBuiltinFunction { name } => {
                hasher.update(b"ResolvedBuiltinFunction");

                hasher.update(b"name");
                hasher.update(name.as_bytes());
            }
            Self::ResolvedFunction { name, is_tail_call } => {
                hasher.update(b"ResolvedFunction");

                hasher.update(b"name");
                hasher.update(name.as_bytes());

                hasher.update(b"is_tail_call");
                hasher.update(&[(*is_tail_call).into()]);
            }
            Self::ResolvedHostFunction { name } => {
                hasher.update(b"ResolvedHostFunction");

                hasher.update(b"name");
                hasher.update(name.as_bytes());
            }
            Self::UnresolvedIdentifier { name } => {
                hasher.update(b"UnresolvedIdentifier");

                hasher.update(b"name");
                hasher.update(name.as_bytes());
            }
            Self::Value(value) => {
                let Value(value) = value;

                hasher.update(b"Value");

                hasher.update(b"value");
                hasher.update(value);
            }
        }
    }
}

impl FragmentHash for FragmentId {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        hasher.update(b"FragmentId");

        let Self { hash } = self;

        hasher.update(b"hash");
        hasher.update(hash.as_bytes());
    }
}

impl FragmentHash for FragmentPayload {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        hasher.update(b"FragmentPayload");

        match self {
            Self::Function { function, next } => {
                hasher.update(b"Function");

                hasher.update(b"function");
                function.hash(hasher);

                hasher.update(b"next");
                next.hash(hasher);
            }
            Self::Expression { expression, next } => {
                hasher.update(b"Expression");

                hasher.update(b"expression");
                expression.hash(hasher);

                hasher.update(b"next");
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
        hasher.update(b"Function");

        let Self { name, branches } = self;

        hasher.update(b"name");
        if let Some(name) = name {
            hasher.update(name.as_bytes());
        }

        hasher.update(b"branches");
        for branch in branches {
            branch.hash(hasher);
        }
    }
}

impl FragmentHash for Parameters {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        hasher.update(b"Parameters");

        let Self { inner } = self;

        hasher.update(b"inner");
        for argument in inner {
            match argument {
                Pattern::Identifier { name } => {
                    hasher.update(b"Identifier");

                    hasher.update(b"name");
                    hasher.update(name.as_bytes());
                }
                Pattern::Literal { value } => {
                    let Value(value) = value;

                    hasher.update(b"Literal");

                    hasher.update(b"value");
                    hasher.update(value);
                }
            }
        }
    }
}
