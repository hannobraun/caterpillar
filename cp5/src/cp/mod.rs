mod pipeline;
mod data_stack;

pub use self::data_stack::{DataStack, DataStackError};

pub fn execute(code: &str, data_stack: &mut DataStack) {
    for word in code.split_whitespace() {
        if word == "true" {
            data_stack.push(true);
        }
    }
}
