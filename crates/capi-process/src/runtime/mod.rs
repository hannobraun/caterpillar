mod instructions;
mod location;
mod value;

pub use self::{
    instructions::{Instruction, Instructions},
    location::Location,
    value::Value,
};

pub use super::operands::Operands;
