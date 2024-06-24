use capi_process::Value;

/// Memory that Caterpillar code can access
///
/// # Implementation Notes
///
/// This type doesn't really fit in any of the current crates:
///
/// - It doesn't fit in `capi-process`, since memory access is not part of the
///   core language. It is a host-specific concept.
/// - It doesn't fit in `capi-protocol` (where, as of this writing, it is
///   located), as it's not really part of the communication protocol.
/// - It can't live in the runtime crate (where the code that makes it work is
///   located), because `capi-protocol` can't depend on that, but the protocol
///   types need to be able to use `Memory`.
///
/// This is part of a larger problem, that host-specific concepts are not
/// cleanly separated from other concerns. For example, `capi-process` has types
/// that encode a lot of knowledge about input and graphics, while not being
/// responsible for handling any of that.
///
/// All of this isn't critical, as only one host exists. At some point, more of
/// them will be supported, and then all of this will need to get figured out.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Memory {
    #[serde(with = "serde_big_array::BigArray")]
    pub inner: [Value; 256],
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            inner: [Value(0); 256],
        }
    }
}
