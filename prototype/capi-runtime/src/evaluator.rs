use std::iter;

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

        let program = [b't'];
        code[..program.len()].copy_from_slice(&program);

        Self { code, data }
    }

    pub fn evaluate(
        &mut self,
        arguments: impl IntoIterator<Item = u8>,
    ) -> &[u8] {
        let code_ptr = 0;
        let mut stack = Stack {
            ptr: self.data.len() - 1,
        };

        for b in arguments {
            self.data[stack.ptr] = b;
            stack.ptr -= 1;
        }

        loop {
            let instruction = self.code[code_ptr];

            match instruction {
                // Terminate the program
                b't' => {
                    break;
                }

                opcode => {
                    let opcode_as_char: char = opcode.into();
                    panic!("Unknown opcode: `{opcode_as_char}` ({opcode:#x})");
                }
            }
        }

        &self.data
    }
}

/// A downward-growing stack
struct Stack {
    // Points to the address where the *next* item will be pushed
    ptr: usize,
}
