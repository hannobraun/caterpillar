use capi_process::Value;

use crate::syntax::Pattern;

use super::{
    Branch, Fragment, FragmentId, FragmentKind, Function, Parameters, Payload,
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
/// name under which the type is imported into the module with all the
/// implementations. (That last bit prevents collisions between types with the
/// same names.)
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
///
/// **4. Hashing any field must start with a unique string.**
///
/// Hashing _any_ field, whether from a struct or enum variant, and regardless
/// if that happens to follow struct or tuple style, must start by feeding a
/// unique string to the hasher.
///
/// If subsequent fields contain similar data, for example lists of strings, but
/// there is no unique string to serve as a separator between those lists, then
/// moving the end of the first list to the beginning of the second, for
/// example, would result in the same hash. We prevent this case by using a
/// unique string as a separator between the lists.
///
/// ## Limitations
///
/// Rule 4 can be circumvented, if struct fields contain data that is equal to a
/// field name. It would probably be better to generate long, random strings to
/// use instead of field names, but it's unclear how to generate those. (Every
/// contributor should be able to do this, so we'd need tooling in this
/// repository.)
///
/// Either way, with or without random strings, this scheme won't stand up to
/// any effort by a motivated attacher, and must not be relied upon for this
/// purpose.
///
/// ## Implementation Note
///
/// It would be nice to automate this via a derive macro, but that would have to
/// be written, and then require ongoing maintenance. It's unclear if and when
/// doing so would actually be an advantage.
///
/// In addition, if random strings were used as per the discussion in the
/// section on limitations, we'd have to store the random strings in the
/// repository and make them accessible to the derive macro. Otherwise, hashes
/// would not be stable over builds, making them unsuitable for some of the
/// intended use cases.
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

        let Self { parent, kind } = self;

        hasher.update(b"parent");
        if let Some(parent) = parent {
            parent.hash(hasher);
        }

        hasher.update(b"kind");
        kind.hash(hasher);
    }
}

impl FragmentHash for Payload {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        hasher.update(b"FragmentExpression");

        match self {
            Self::CallToFunction { name, is_tail_call } => {
                hasher.update(b"ResolvedFunction");

                hasher.update(b"name");
                hasher.update(name.as_bytes());

                hasher.update(b"is_tail_call");
                hasher.update(&[(*is_tail_call).into()]);
            }
            Self::CallToHostFunction { name } => {
                hasher.update(b"ResolvedHostFunction");

                hasher.update(b"name");
                hasher.update(name.as_bytes());
            }
            Self::CallToIntrinsic {
                intrinsic,
                is_tail_call,
            } => {
                hasher.update(b"Intrinsic");

                hasher.update(b"intrinsic");
                hasher.update(intrinsic.to_string().as_bytes());

                hasher.update(b"is_tail_call");
                hasher.update(&[(*is_tail_call).into()]);
            }
            Self::Comment { text } => {
                hasher.update(b"Comment");

                hasher.update(b"text");
                hasher.update(text.as_bytes());
            }
            Self::Function { function } => {
                hasher.update(b"Block");

                hasher.update(b"function");
                function.hash(hasher);
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

impl FragmentHash for FragmentKind {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        hasher.update(b"FragmentKind");

        match self {
            Self::Payload { payload, next } => {
                hasher.update(b"Payload");

                hasher.update(b"payload");
                payload.hash(hasher);

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

        let Self {
            name,
            branches,
            environment,
        } = self;

        hasher.update(b"name");
        if let Some(name) = name {
            hasher.update(name.as_bytes());
        }

        hasher.update(b"branches");
        for branch in branches {
            branch.hash(hasher);
        }

        hasher.update(b"environment");
        for binding in environment {
            hasher.update(binding.as_bytes());
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
