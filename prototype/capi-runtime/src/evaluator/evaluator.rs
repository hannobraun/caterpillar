use std::iter;

use super::data::Data;

pub struct Evaluator {
    code: Vec<u8>,
    data: Data,
}

impl Evaluator {
    pub fn new(program: &[u8]) -> Self {
        // I want to know when I go beyond certain thresholds, just out of
        // interest. Keeping the limits as low as possible here, to make sure I
        // notice.
        const CODE_SIZE: usize = 32;
        const DATA_SIZE: usize = 8;

        let mut code: Vec<_> = iter::repeat(0).take(CODE_SIZE).collect();
        code[..program.len()].copy_from_slice(&program);

        let data = Data::new(DATA_SIZE);

        Self { code, data }
    }

    pub fn evaluate(
        &mut self,
        arguments: impl IntoIterator<Item = u8>,
    ) -> &[u8] {
        let mut code_ptr = 0;

        for b in arguments {
            self.data.push(b);
        }

        loop {
            let instruction = self.code[code_ptr];

            match instruction {
                // `clone` - Clone the top item on the stack
                b'c' => {
                    let value = self.data.pop();
                    self.data.push(value);
                    self.data.push(value);
                }

                // `push` - Push a value to the stack
                b'p' => {
                    code_ptr += 1;
                    let value = self.code[code_ptr];

                    self.data.push(value);
                }

                // `store` - Store data in memory
                b'S' => {
                    let address = self.data.pop();
                    let value = self.data.pop();

                    self.data.store(address, value);
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

        self.data.read()
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
        let mut evaluator = Evaluator::new(program);
        let data = evaluator.evaluate([]);
        data.to_vec()
    }
}
