use super::data_stack::DataStack;

#[derive(Debug)]
pub struct Evaluator {
    pub call_stack: Vec<usize>,
    pub data_stack: DataStack,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            call_stack: Vec::new(),
            data_stack: DataStack::new(),
        }
    }
}
