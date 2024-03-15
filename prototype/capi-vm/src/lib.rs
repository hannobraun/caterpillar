#![cfg_attr(not(test), no_std)]

mod code;
mod data;
mod evaluator;

pub mod opcode;

pub use self::evaluator::Evaluator;
