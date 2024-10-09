pub mod fragments;
pub mod host;
pub mod intrinsics;
pub mod source_map;
pub mod syntax;

mod compiler;
mod hash;
mod passes;

#[cfg(test)]
mod tests;

pub use self::compiler::Compiler;
