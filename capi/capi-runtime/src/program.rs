use crate::{Evaluator, Functions};

#[derive(Clone)]
pub struct Program {
    pub functions: Functions,
    pub evaluator: Evaluator,
    pub entry: usize,
}

impl Program {
    pub fn push(&mut self, arguments: impl IntoIterator<Item = usize>) {
        for value in arguments {
            self.evaluator.data_stack.push(value);
        }
    }

    pub fn run(&mut self, frame: &mut [u8]) {
        self.evaluator.evaluate(self.entry, frame);
        assert_eq!(self.evaluator.data_stack.num_values(), 0);
    }
}
