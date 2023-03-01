mod bindings;
mod builtins;
mod data_stack;
mod evaluator;
mod functions;
mod parser;
mod tokenizer;

pub use self::{
    bindings::Bindings,
    data_stack::{DataStack, Value},
    evaluator::evaluate,
    functions::Functions,
    parser::{parse, Expression, Expressions},
    tokenizer::{tokenize, Token, Tokens},
};

pub struct Interpreter {
    pub functions: Functions,
    pub data_stack: DataStack,
    pub bindings: Bindings,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            functions: Functions::new(),
            data_stack: DataStack::new(),
            bindings: Bindings::new(),
        }
    }
}
