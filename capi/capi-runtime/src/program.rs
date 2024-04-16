use crate::{evaluator::EvaluatorState, syntax::Syntax, Evaluator, Functions};

#[derive(Clone, Default)]
pub struct Program {
    pub functions: Functions,
    pub evaluator: Evaluator,
    pub entry: usize,
}

impl Program {
    pub fn define(&mut self, name: &str, f: impl FnOnce(&mut Syntax)) {
        self.functions.define(name, f)
    }

    pub fn push(&mut self, arguments: impl IntoIterator<Item = usize>) {
        for value in arguments {
            self.evaluator.data_stack.push(value);
        }
    }

    pub fn reset(&mut self) {
        self.evaluator.instruction = self.entry;
    }

    pub fn step(&mut self, mem: &mut [u8]) -> EvaluatorState {
        self.evaluator.step(mem)
    }
}
