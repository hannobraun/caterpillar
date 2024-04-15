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
        frame_width: usize,
        frame_height: usize,
        frame: &mut [u8],
    ) {
        self.evaluator.data_stack.push(frame_width);
        self.evaluator.data_stack.push(frame_height);
        self.evaluator.evaluate(self.entry, frame);

        assert_eq!(self.evaluator.data_stack.num_values(), 0);
    }
}
