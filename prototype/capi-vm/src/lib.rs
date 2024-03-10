#![no_std]

mod data;
mod evaluator;
mod word;

pub mod opcode;
pub mod width;

pub use self::evaluator::Evaluator;
