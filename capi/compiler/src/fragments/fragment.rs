use capi_runtime::Value;

use crate::intrinsics::Intrinsic;

use super::{FoundFunction, FragmentMap, Function, Hash};

/// # A content-addressed piece of code
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub enum Fragment {
    /// # A call to a user-defined function
    ///
    /// ## Implementation Note
    ///
    /// This enum variant references the function by name. It should instead
    /// reference the function using an `id: FragmentId` field.
    ///
    /// This would have the advantage of versioning this call. It could refer to
    /// any available version of the function, which is a useful feature to have
    /// for many reasons.
    ///
    /// Unfortunately, this is not easy. There are two main hurdles, as best I
    /// can tell:
    ///
    /// 1. It requires function fragments to be created in the correct order, as
    ///    the called function must be created before its caller.
    /// 2. There would need to be special handling of recursive calls, or there
    ///    would be a dependency cycle when hashing the calls and their targets.
    ///
    /// I think what we need, is a new compiler pass that creates a call graph.
    /// This call graph can then be used to order the creation of fragments,
    /// from the leaves up, as well as to detect any recursive call cycles.
    ///
    /// As for the handling of those, here is some information on how Unison
    /// does that, which might prove useful:
    /// https://stackoverflow.com/a/73343072/8369834
    CallToFunction {
        name: String,

        /// # Indicate whether the call is in tail position
        ///
        /// This is relevant as function calls might necessitate tail call
        /// elimination.
        is_tail_call: bool,
    },

    /// # A call to a function defined by the host
    ///
    /// Host functions present as functions to the user. But contrary to regular
    /// functions, they have no representation in the form of Caterpillar code.
    ///
    /// The compiler translates calls to host functions into instructions that
    /// trigger a specific effect. This effect is then handled by the host in
    /// whatever way it deems appropriate.
    CallToHostFunction {
        effect_number: u8,
    },

    /// # A call to a compiler intrinsic
    ///
    /// Compiler intrinsics present as functions to the user. But contrary to
    /// regular functions, they have no representation in the form of
    /// Caterpillar code.
    ///
    /// The compiler translates calls to intrinsics directly into whichever
    /// instructions are required for the specific intrinsic.
    CallToIntrinsic {
        intrinsic: Intrinsic,

        /// # Indicate whether the call is in tail position
        ///
        /// This is relevant, as intrinsics can trigger function calls, which
        /// might necessitate tail call elimination.
        is_tail_call: bool,
    },

    Comment {
        text: String,
    },

    /// # A function literal
    ///
    /// This is used to represent both anonymous functions that are used where
    /// an expression is accepted, as well as named functions defined in the
    /// top-level context.
    Function {
        function: Function,
    },

    ResolvedBinding {
        name: String,
    },
    UnresolvedIdentifier {
        name: String,
    },
    Value(Value),

    /// # This fragment is a terminator
    ///
    /// Terminators carry no payload and, well, terminate any context in which
    /// fragments reside. The reason for this is explained in the documentation
    /// of [`Fragment`]'s `parent` field.
    Terminator,
}

impl Fragment {
    pub fn as_call_to_function<'r>(
        &self,
        fragments: &'r FragmentMap,
    ) -> Option<FoundFunction<'r>> {
        let Fragment::CallToFunction { name, .. } = &self else {
            return None;
        };

        let function = fragments.find_function_by_name(name).expect(
            "Got function name from fragment that calls it; expecting it to \
            exist.",
        );

        Some(function)
    }

    pub fn as_comment(&self) -> Option<&String> {
        let Fragment::Comment { text } = &self else {
            return None;
        };

        Some(text)
    }
}

/// # A unique identifier for a fragment
///
/// A fragment is identified by its contents, but also by its position within
/// the code.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct FragmentId {
    /// # The fragment's parent
    ///
    /// Refers to the fragment's parent fragment. If the fragment resides in the
    /// root context, then it has no parent.
    ///
    /// All other fragments have a parent. By convention, this is the fragment
    /// _after_ the function that the fragment resides in (i.e. the `next`
    /// fragment of that function).
    ///
    /// This must be so, because by the time that a fragment is constructed, the
    /// function fragment for the function it resides in, or any fragments
    /// preceding that, are not constructed yet. Thus, they do not have an ID
    /// that can be used to refer to them.
    ///
    /// Any _succeeding_ fragments, on the other hand, are already constructed.
    /// Therefore, the `next` fragment of the function fragment can stand in as
    /// the parent.
    ///
    /// Function fragments always have a `next` fragment that can be used in
    /// this way. This is that reason that terminators exist, to make sure of
    /// that.
    pub parent: Option<Hash<FragmentId>>,

    /// # The next fragment within the fragment's context
    ///
    /// Every fragment resides in a context, either the root context or a
    /// function. Every payload-carrying fragment has a fragment that follows it
    /// within that context, which is either another payload-carrying fragment,
    /// or a terminator.
    ///
    /// Might be `None`, if the fragment is a terminator.
    pub next: Option<Hash<FragmentId>>,

    /// # The fragment itself
    pub this: Hash<Fragment>,
}

impl FragmentId {
    /// # Compute the hash of this location
    pub(crate) fn hash(&self) -> Hash<Self> {
        Hash::new(self)
    }
}
