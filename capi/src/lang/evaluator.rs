use super::data_stack::DataStack;

#[derive(Debug)]
pub struct Evaluator {
    pub call_stack: Vec<usize>,
    pub data_stack: DataStack,
}
