mod builtins;
mod data_stack;
mod evaluator;
mod functions;
mod parser;
mod tokenizer;

pub use self::{
    data_stack::{DataStack, Type, Value},
    evaluator::evaluate,
    functions::{Arg, Functions},
    parser::{parse, Expression, Expressions},
    tokenizer::{tokenize, Token, Tokens},
};

pub struct Interpreter {
    pub functions: Functions,
    pub data_stack: DataStack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            functions: Functions::new(),
            data_stack: DataStack::new(),
        }
    }
}
