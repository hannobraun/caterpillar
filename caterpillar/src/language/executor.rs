use std::collections::VecDeque;

use super::{evaluator::Operation, values::Value};

pub fn execute(
    ops: impl Iterator<Item = Operation>,
    stack: &mut VecDeque<Value>,
) {
    for op in ops {
        match op {
            Operation::Push(value) => {
                stack.push_back(value);
            }
        }
    }
}
