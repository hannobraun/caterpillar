use crate::{Evaluator, Functions};

#[derive(Clone)]
pub struct Program {
    pub functions: Functions,
    pub evaluator: Evaluator,
    pub entry: usize,
}

impl Program {
    pub fn run(
        &mut self,
        arguments: impl IntoIterator<Item = usize>,
        frame: &mut [u8],
    ) {
        for value in arguments {
            self.evaluator.data_stack.push(value);
        }

        self.evaluator.evaluate(self.entry, frame);

        assert_eq!(self.evaluator.data_stack.num_values(), 0);
    }
}
