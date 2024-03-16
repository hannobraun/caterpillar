// I don't see the point of this lint. That weird case they present in its
// documentation certainly doesn't apply to me, and I've also never seen it
// anywhere else.
//
// We have a `clippy.toml` that is supposed to allow this for private modules,
// but it doesn't seem to work. Or I'm holding it wrong. I don't know.
#![allow(clippy::module_inception)]

pub mod pipeline;
pub mod platform;
pub mod repr;
pub mod runtime;
