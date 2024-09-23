use capi_runtime::Value;

use crate::intrinsics::Intrinsic;

use super::{FoundFunction, FragmentMap, Function};

/// # A pre-compiled piece of code
///
/// Fragments are the core of Caterpillar's code representation, the smallest
/// units of code.
///
/// They are the result of a partial compilation process. This is called
/// pre-compilation, because it happens before the actual translation into
/// instructions that the runtime can interpret.
///
///
/// ## Error Handling
///
/// An important feature of this code representation is, that it can be the
/// result of a failed compilation process. If, for example, an identifier can't
/// be resolved, this is still encoded as a fragment.
///
/// As a result, other code that is not affected can still be executed (as part
/// of automated testing, for example). But also, the rich representation
/// produced by the pre-compilation process is still available for display by
/// tooling, regardless of any isolated errors.
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
    CallToFunction {
        /// # The name of the function being called
        ///
        /// ## Implementation Note
        ///
        /// This references references the called function by name. It should
        /// instead reference it using an `id: FragmentId` field.
        ///
        /// This would have the advantage of versioning this call. It could
        /// refer to any available version of the function, which is a useful
        /// feature to have for many reasons.
        ///
        /// Unfortunately, this is not easy. There are two main hurdles, as
        /// best I can tell:
        ///
        /// 1. It requires function fragments to be created in the correct
        ///    order, as the called function must be created before its caller.
        /// 2. There would need to be special handling of recursive calls, or
        ///    there would be a dependency cycle when hashing the calls and
        ///    their targets.
        ///
        /// I think what we need, is a new compiler pass that creates a call
        /// graph. This call graph can then be used to order the creation of
        /// fragments, from the leaves up, as well as to detect any recursive
        /// call cycles.
        ///
        /// As for the handling of those, here is some information on how Unison
        /// does that, which might prove useful:
        /// https://stackoverflow.com/a/73343072/8369834
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
        /// # A number that identifies the specific effect
        ///
        /// The meaning of this number is only known to the host. The compiler
        /// doesn't know, nor doesn't need to know, what it means.
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
        /// # The intrinsic being called
        intrinsic: Intrinsic,

        /// # Indicate whether the call is in tail position
        ///
        /// This is relevant, as intrinsics can trigger function calls, which
        /// might necessitate tail call elimination.
        is_tail_call: bool,
    },

    /// # A comment, which does not influence the execution of the code
    Comment {
        /// # The text of the comment
        text: String,
    },

    /// # A function literal
    ///
    /// This is used to represent both anonymous functions that are used where
    /// an expression is accepted, as well as named functions defined in the
    /// top-level context.
    Function {
        /// # The function defined by this literal
        function: Function,
    },

    /// # A reference to a binding
    ///
    /// The reference to the binding has been resolved, meaning it is available
    /// in the fragment's scope.
    ResolvedBinding {
        /// # The name of the binding
        name: String,
    },

    /// # An unresolved identifier
    ///
    /// This is the result of a compiler error.
    UnresolvedIdentifier {
        /// # The name of the unresolved identifier
        name: String,
    },

    /// # A literal value
    Value(Value),

    /// # A terminator
    ///
    /// Terminators carry no payload. Every context (which can either be a
    /// function, or the top-level context) is concluded with a terminator.
    ///
    /// Every fragment that is not in the root context (which means it is in a
    /// function) has a parent. Per convention, this is the fragment _after_ the
    /// fragment that represents the function literal.
    ///
    /// (It can't be the function fragment itself, as that is going to get a
    /// hash that depends on the fragments within it. Using it as the parent
    /// would create a circular dependency when doing the hashing.)
    ///
    /// Making sure that every context has a terminator, means that there is
    /// always a parent available for every fragment.
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
