#![no_std]

mod data;
mod evaluator;

pub mod opcode;
pub mod width;

pub use self::evaluator::Evaluator;
