use std::iter;

use crate::ffi_in::{CODE_SIZE, DATA_SIZE};

use super::data::Data;

pub struct Evaluator {
    code: Vec<u8>,
    data: Data,
}

impl Evaluator {
    pub fn new() -> Self {
        let code = iter::repeat(0).take(CODE_SIZE).collect();
        let data = Data::new(DATA_SIZE);

        Self { code, data }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.code[..program.len()].copy_from_slice(&program);
    }

    pub fn push_arguments(
        &mut self,
        arguments: impl IntoIterator<Item = u8>,
        data_memory: &mut [u8],
    ) {
        for b in arguments {
            self.data.push(b, data_memory);
        }
    }

    pub fn evaluate(&mut self, data_memory: &mut [u8]) {
        let mut code_ptr = 0;

        loop {
            let instruction = self.code[code_ptr];

            match instruction {
                // `clone` - Clone the top item on the stack
                b'c' => {
                    let value = self.data.pop(data_memory);
                    self.data.push(value, data_memory);
                    self.data.push(value, data_memory);
                }

                // `push` - Push a value to the stack
                b'p' => {
                    code_ptr += 1;
                    let value = self.code[code_ptr];

                    self.data.push(value, data_memory);
                }

                // `store` - Store data in memory
                b'S' => {
                    let address = self.data.pop(data_memory);
                    let value = self.data.pop(data_memory);

                    self.data.store(address, value, data_memory);
                }

                // `terminate` - Terminate the program
                b't' => {
                    break;
                }

                opcode => {
                    let opcode_as_char: char = opcode.into();
                    panic!("Unknown opcode: `{opcode_as_char}` ({opcode:#x})");
                }
            }

            code_ptr += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ffi_in::DATA_SIZE;

    use super::Evaluator;

    #[test]
    fn clone() {
        let data = evaluate(&[b'p', 255, b'c', b't']);
        assert_eq!(data[data.len() - 2..], [255, 255]);
    }

    #[test]
    fn push() {
        let data = evaluate(&[b'p', 255, b't']);
        assert_eq!(data[data.len() - 1..], [255]);
    }

    #[test]
    fn store() {
        let data = evaluate(&[b'p', 255, b'p', 0, b'S', b't']);
        assert_eq!(data[..1], [255]);
    }

    #[test]
    fn terminate() {
        evaluate(&[b't']);
        // This should not run forever, or cause any kind of panic.
    }

    fn evaluate(program: &[u8]) -> [u8; DATA_SIZE] {
        let mut data_memory = [0; DATA_SIZE];

        let mut evaluator = Evaluator::new();
        evaluator.load_program(program);

        evaluator.evaluate(&mut data_memory);
        data_memory
    }
}
