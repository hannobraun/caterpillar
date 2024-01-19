// I don't see the point of this lint. That weird case they present in its
// documentation certainly doesn't apply to me, and I've also never seen it
// anywhere else.
//
// We have a `clippy.toml` that is supposed to allow this for private modules,
// but it doesn't seem to work. Or I'm holding it wrong. I don't know.
#![allow(clippy::module_inception)]

pub mod args;
pub mod display;
pub mod loader;
pub mod platform;
pub mod thread;

pub use self::thread::DesktopThread;

/// Re-definition of [`capi_core::Interpreter`] with a desktop context
///
/// `capi-core` has a (quasi-circular) dev-dependency on this crate, for the
/// test suite. I'm saying "quasi", because it's not really circular. The
/// dependency chain goes like this:
///
/// "regular `capi-core`" <- "regular `capi-desktop" <- "test-mode `capi-core`"
///
/// That is a very important distinction, because the two different build
/// configurations of `capi-core` mean that, as far as the type system is
/// concerned, those are essentially different crates.
///
/// As a consequence, if the test suite constructs a `crate::Interpreter` (test)
/// and passes it to a `capi-desktop` function which expects a
/// `capi_core::Interpreter` (regular), this will result in a "mismatched types"
/// error.
///
/// With this type definition, we make `capi-desktop`'s understanding of what
/// `Interpreter` is available to the `capi-core` test suite, resolving the type
/// error.
pub type Interpreter = capi_core::Interpreter<platform::PlatformContext>;
