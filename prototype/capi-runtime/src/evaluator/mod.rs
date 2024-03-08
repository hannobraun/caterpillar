mod data;

use std::iter;

use self::data::Data;

pub struct Evaluator {
    code: Vec<u8>,
    data: Vec<u8>,
}

impl Evaluator {
    pub fn new() -> Self {
        // I want to know when I go beyond certain thresholds, just out of
        // interest. Keeping the limits as low as possible here, to make sure I
        // notice.
        const CODE_SIZE: usize = 4;
        const DATA_SIZE: usize = 2;

        let mut code: Vec<_> = iter::repeat(0).take(CODE_SIZE).collect();
        let data = iter::repeat(0).take(DATA_SIZE).collect();

        let program = [b'p', 0, b'S', b't'];
        code[..program.len()].copy_from_slice(&program);

        Self { code, data }
    }

    pub fn evaluate(
        &mut self,
        arguments: impl IntoIterator<Item = u8>,
    ) -> &[u8] {
        let mut code_ptr = 0;
        let mut stack = Data::new(&mut self.data);

        for b in arguments {
            stack.push(b);
        }

        loop {
            let instruction = self.code[code_ptr];

            match instruction {
                // `push` - Push a value to the stack
                b'p' => {
                    code_ptr += 1;
                    let value = self.code[code_ptr];

                    stack.push(value);
                }

                // `store` - Store data in memory
                b'S' => {
                    let address = stack.pop();
                    let value = stack.pop();

                    stack.store(address, value);
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

        &self.data
    }
}
