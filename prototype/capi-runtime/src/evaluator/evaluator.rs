use super::data::Data;

pub struct Evaluator {
    data: Data,
}

impl Evaluator {
    pub fn new(data: &[u8]) -> Self {
        let data = Data::new(&data);
        Self { data }
    }

    pub fn load_program(&mut self, program: &[u8], code: &mut [u8]) {
        code[..program.len()].copy_from_slice(&program);
    }

    pub fn push_args(
        &mut self,
        args: impl IntoIterator<Item = u8>,
        data: &mut [u8],
    ) {
        for b in args {
            self.data.push(b, data);
        }
    }

    pub fn evaluate(&mut self, code: &[u8], data: &mut [u8]) {
        let mut code_ptr = 0;

        loop {
            let instruction = code[code_ptr];

            match instruction {
                // `clone` - Clone the top item on the stack
                b'c' => {
                    let value = self.data.pop(data);
                    self.data.push(value, data);
                    self.data.push(value, data);
                }

                // `push` - Push a value to the stack
                b'p' => {
                    code_ptr += 1;
                    let value = code[code_ptr];

                    self.data.push(value, data);
                }

                // `store` - Store data in memory
                b'S' => {
                    let address = self.data.pop(data);
                    let value = self.data.pop(data);

                    self.data.store(address, value, data);
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
    use crate::ffi_in::{CODE_SIZE, DATA_SIZE};

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
        let mut code = [0; CODE_SIZE];
        let mut data = [0; DATA_SIZE];

        let mut evaluator = Evaluator::new(&data);
        evaluator.load_program(program, &mut code);

        evaluator.evaluate(&mut code, &mut data);
        data
    }
}
