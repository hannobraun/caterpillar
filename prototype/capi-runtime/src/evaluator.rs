use std::iter;

pub struct Evaluator {
    code: Vec<u8>,
}

impl Evaluator {
    pub fn new() -> Self {
        // I want to know when I go beyond certain thresholds, just out of
        // interest. Keeping the limits as low as possible here, to make sure I
        // notice.
        const CODE_SIZE: usize = 1;

        let mut code: Vec<_> = iter::repeat(0).take(CODE_SIZE).collect();

        let program = [b't'];
        code.copy_from_slice(&program);

        Self { code }
    }

    pub fn evaluate(&mut self) {
        let code_ptr = 0;

        loop {
            let instruction = self.code[code_ptr];

            match instruction {
                // Terminate the program
                b't' => {
                    break;
                }

                opcode => panic!("Unknown opcode: `{opcode}`"),
            }
        }
    }
}
