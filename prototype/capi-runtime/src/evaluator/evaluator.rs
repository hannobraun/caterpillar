use std::iter;

use crate::ffi_in::{CODE_SIZE, DATA_SIZE};

use super::data::Data;

pub struct Evaluator {
    code: Vec<u8>,
    data: Data,
    data_memory: Vec<u8>,
}

impl Evaluator {
    pub fn new() -> Self {
        let code = iter::repeat(0).take(CODE_SIZE).collect();
        let data_memory: Vec<_> = iter::repeat(0).take(DATA_SIZE).collect();
        let data = Data::new(DATA_SIZE);

        Self {
            code,
            data,
            data_memory,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.code[..program.len()].copy_from_slice(&program);
    }

    pub fn evaluate(
        &mut self,
        arguments: impl IntoIterator<Item = u8>,
    ) -> &[u8] {
        let mut code_ptr = 0;

        for b in arguments {
            self.data.push(b, &mut self.data_memory);
        }

        loop {
            let instruction = self.code[code_ptr];

            match instruction {
                // `clone` - Clone the top item on the stack
                b'c' => {
                    let value = self.data.pop(&mut self.data_memory);
                    self.data.push(value, &mut self.data_memory);
                    self.data.push(value, &mut self.data_memory);
                }

                // `push` - Push a value to the stack
                b'p' => {
                    code_ptr += 1;
                    let value = self.code[code_ptr];

                    self.data.push(value, &mut self.data_memory);
                }

                // `store` - Store data in memory
                b'S' => {
                    let address = self.data.pop(&mut self.data_memory);
                    let value = self.data.pop(&mut self.data_memory);

                    self.data.store(address, value, &mut self.data_memory);
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

        &self.data_memory
    }
}

#[cfg(test)]
mod tests {
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

    fn evaluate(program: &[u8]) -> Vec<u8> {
        let mut evaluator = Evaluator::new();
        evaluator.load_program(program);

        let data = evaluator.evaluate([]);
        data.to_vec()
    }
}
