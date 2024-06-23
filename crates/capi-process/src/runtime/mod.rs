mod function;
mod instructions;
mod location;
mod operands;
mod value;

pub use self::{
    function::Function,
    instructions::{Instruction, Instructions},
    location::Location,
    operands::{MissingOperand, Operands},
    value::Value,
};

pub use super::stack::Stack;
