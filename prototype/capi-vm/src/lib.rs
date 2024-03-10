#![no_std]

mod argument;
mod data;
mod evaluator;

pub mod opcode;
pub mod width;

pub use self::evaluator::Evaluator;
