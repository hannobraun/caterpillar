//! End-to-end testing for `capi-compiler` and `capi-process`
//!
//! That this module lives in `capi-compiler` is a practical decision. The crate
//! depends on `capi-process` anyway, so we have everything here that we need.
//!
//! But it's a bit weird, because these tests explicitly cover both crates. And
//! in the future, when we can do hot code reloading, we'll need tests for that
//! too. It's not clear to me whether those should then go somewhere else, or if
//! we then need a central place for all of them.
