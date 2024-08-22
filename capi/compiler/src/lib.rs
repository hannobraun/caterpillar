pub mod fragments;
pub mod host;
pub mod intrinsics;
pub mod source_map;
pub mod syntax;

mod compile;
mod passes;

#[cfg(test)]
mod tests;

pub use self::compile::compile;
