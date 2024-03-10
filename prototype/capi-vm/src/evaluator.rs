use crate::{opcode, width, word::Word};

use super::data::Data;

pub struct Evaluator {
    data: Data,
}

impl Evaluator {
    pub fn new(data: &[u8]) -> Self {
        let data = Data::new(data);
        Self { data }
    }

    pub fn push_argument<const N: usize>(
        &mut self,
        argument: impl Word<N>,
        data: &mut [u8],
    ) {
        for b in argument.to_bytes() {
            self.data.push([b], data);
        }
    }

    pub fn evaluate(&mut self, code: &[u8], data: &mut [u8]) {
        let mut code_ptr = 0;

        loop {
            let Some(&instruction) = code.get(code_ptr) else {
                break;
            };

            let opcode = instruction & 0x3f;
            let width = instruction & 0xc0;

            match opcode {
                opcode::TERMINATE => {
                    break;
                }
                opcode::PUSH => {
                    code_ptr += 1;
                    let value = code[code_ptr];

                    self.data.push([value], data);
                }
                opcode::DROP => {
                    if width == width::W8 {
                        self.data.pop::<1>(data);
                    }
                    if width == width::W16 {
                        self.data.pop::<2>(data);
                    }
                    if width == width::W32 {
                        self.data.pop::<4>(data);
                    }
                    if width == width::W64 {
                        self.data.pop::<8>(data);
                    }
                }
                opcode::STORE => {
                    let [address] = self.data.pop(data);
                    let [value] = self.data.pop(data);

                    self.data.store(address, value, data);
                }
                opcode::CLONE => {
                    let [value] = self.data.pop(data);
                    self.data.push([value], data);
                    self.data.push([value], data);
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
    use crate::{opcode, width};

    use super::Evaluator;

    #[test]
    fn terminate() {
        evaluate([opcode::TERMINATE], [], []);
        // This should not run forever, nor cause any kind of panic.
    }

    #[test]
    fn push() {
        let data = evaluate([opcode::PUSH, 255], [0], []);
        assert_eq!(data, [255]);
    }

    #[test]
    fn drop8() {
        let data =
            evaluate([opcode::DROP | width::W8, opcode::PUSH, 255], [0], [127]);
        assert_eq!(data, [255]);
    }
    #[test]
    fn drop16() {
        let data = evaluate(
            [opcode::DROP | width::W16, opcode::PUSH, 255],
            [0, 0],
            [127, 127],
        );
        assert_eq!(data, [127, 255]);
    }

    #[test]
    fn drop32() {
        let data = evaluate(
            [opcode::DROP | width::W32, opcode::PUSH, 255],
            [0, 0, 0, 0],
            [127, 127, 127, 127],
        );
        assert_eq!(data, [127, 127, 127, 255]);
    }

    #[test]
    fn drop64() {
        let data = evaluate(
            [opcode::DROP | width::W64, opcode::PUSH, 255],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [127, 127, 127, 127, 127, 127, 127, 127],
        );
        assert_eq!(data, [127, 127, 127, 127, 127, 127, 127, 255]);
    }

    #[test]
    fn store() {
        let data = evaluate([opcode::STORE], [0, 0, 0], [255, 0]);
        assert_eq!(data, [255, 0, 255]);
    }

    #[test]
    fn clone() {
        let data = evaluate([opcode::CLONE], [0, 0], [255]);
        assert_eq!(data, [255, 255]);
    }

    fn evaluate<const C: usize, const D: usize, const A: usize>(
        code: [u8; C],
        mut data: [u8; D],
        args: [u8; A],
    ) -> [u8; D] {
        let mut evaluator = Evaluator::new(&data);

        for arg in args {
            evaluator.push_argument(arg, &mut data);
        }

        evaluator.evaluate(&code, &mut data);
        data
    }
}
