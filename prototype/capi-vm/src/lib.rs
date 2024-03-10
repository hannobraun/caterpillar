#![no_std]

mod argument;
mod data;
mod evaluator;

pub mod opcode;

pub use self::evaluator::Evaluator;
