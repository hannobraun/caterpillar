mod data_stack;
mod function;

pub use self::{
    data_stack::{DataStack, StackUnderflow, Value},
    function::Function,
};
