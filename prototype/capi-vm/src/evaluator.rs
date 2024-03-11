use crate::{
    opcode,
    width::{Width, W16, W32, W64, W8},
};

use super::data::Data;

pub struct Evaluator {
    data: Data,
}

impl Evaluator {
    pub fn new(data: &[u8]) -> Self {
        let data = Data::new(data);
        Self { data }
    }

    pub fn push_u8(&mut self, value: u8, data: &mut [u8]) -> &mut Self {
        self.data.push([value], data);
        self
    }

    pub fn push_u16(&mut self, value: u16, data: &mut [u8]) -> &mut Self {
        self.data.push(value.to_le_bytes(), data);
        self
    }

    pub fn push_u32(&mut self, value: u32, data: &mut [u8]) -> &mut Self {
        self.data.push(value.to_le_bytes(), data);
        self
    }

    pub fn push_u64(&mut self, value: u64, data: &mut [u8]) -> &mut Self {
        self.data.push(value.to_le_bytes(), data);
        self
    }

    pub fn evaluate(&mut self, code: &[u8], data: &mut [u8]) {
        let mut code_ptr = 0;

        loop {
            let Some(&instruction) = code.get(code_ptr) else {
                break;
            };

            let opcode = instruction & 0x3f;
            let width = instruction & 0xc0;

            let width = match width {
                W8::FLAG => W8::INFO,
                W16::FLAG => W16::INFO,
                W32::FLAG => W32::INFO,
                W64::FLAG => W64::INFO,
                _ => unreachable!("2 bits can encode 4 values"),
            };

            match opcode {
                opcode::TERMINATE => {
                    break;
                }
                opcode::PUSH => {
                    let mut buffer = [0; W64::SIZE];
                    let value = &mut buffer[0..width.size];

                    for b in value {
                        code_ptr += 1;
                        *b = code[code_ptr];
                    }

                    self.data.push(buffer.into_iter().take(width.size), data);
                }
                opcode::DROP => {
                    let mut buffer = [0; 8];
                    let value = &mut buffer[..width.size];
                    self.data.pop(value, data);
                }
                opcode::STORE => {
                    let address = {
                        let mut bytes = [0; 4];
                        self.data.pop(&mut bytes, data);

                        u32::from_le_bytes(bytes)
                    };

                    let mut buffer = [0; 8];
                    let value = &mut buffer[..width.size];
                    self.data.pop(value, data);

                    self.data.store(address, value.iter().copied(), data);
                }
                opcode::CLONE => {
                    let value = {
                        let mut bytes = [0; 1];
                        self.data.pop(&mut bytes, data);

                        let [value] = bytes;
                        value
                    };

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
    use crate::{
        opcode,
        width::{Width, W16, W32, W64, W8},
    };

    use super::Evaluator;

    #[test]
    fn terminate() {
        let mut data = [];
        let mut evaluator = Evaluator::new(&data);

        evaluator.evaluate(&[opcode::TERMINATE], &mut data);
        // This should not run forever, nor cause any kind of panic.
    }

    #[test]
    fn push8() {
        let mut data = [0; 1];
        let mut evaluator = Evaluator::new(&data);

        evaluator.evaluate(&[opcode::PUSH | W8::FLAG, 0x11], &mut data);

        assert_eq!(data, [0x11]);
    }

    #[test]
    fn push16() {
        let mut data = [0; 2];
        let mut evaluator = Evaluator::new(&data);

        evaluator.evaluate(&[opcode::PUSH | W16::FLAG, 0x11, 0x22], &mut data);

        assert_eq!(data, [0x11, 0x22]);
    }

    #[test]
    fn push32() {
        let mut data = [0; 4];
        let mut evaluator = Evaluator::new(&data);

        evaluator.evaluate(
            &[opcode::PUSH | W32::FLAG, 0x11, 0x22, 0x33, 0x44],
            &mut data,
        );

        assert_eq!(data, [0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn push64() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator.evaluate(
            &[
                opcode::PUSH | W64::FLAG,
                0x11,
                0x22,
                0x33,
                0x44,
                0x55,
                0x66,
                0x77,
                0x88,
            ],
            &mut data,
        );

        assert_eq!(data, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
    }

    #[test]
    fn drop8() {
        let mut data = [0; 1];
        let mut evaluator = Evaluator::new(&data);

        evaluator.push_u8(0x11, &mut data).evaluate(
            &[opcode::DROP | W8::FLAG, opcode::PUSH, 0x22],
            &mut data,
        );

        assert_eq!(data, [0x22]);
    }

    #[test]
    fn drop16() {
        let mut data = [0; 2];
        let mut evaluator = Evaluator::new(&data);

        evaluator.push_u16(0x1111, &mut data).evaluate(
            &[opcode::DROP | W16::FLAG, opcode::PUSH, 0x22],
            &mut data,
        );

        assert_eq!(data, [0x11, 0x22]);
    }

    #[test]
    fn drop32() {
        let mut data = [0; 4];
        let mut evaluator = Evaluator::new(&data);

        evaluator.push_u32(0x11111111, &mut data).evaluate(
            &[opcode::DROP | W32::FLAG, opcode::PUSH, 0x22],
            &mut data,
        );

        assert_eq!(data, [0x11, 0x11, 0x11, 0x22]);
    }

    #[test]
    fn drop64() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator.push_u64(0x1111111111111111, &mut data).evaluate(
            &[opcode::DROP | W64::FLAG, opcode::PUSH, 0x22],
            &mut data,
        );

        assert_eq!(data, [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22]);
    }

    #[test]
    fn store8() {
        let data = evaluate(
            [opcode::STORE | W8::FLAG],
            [0, 0, 0, 0, 0, 0, 0],
            [0x11, 0, 0, 0, 1],
        );
        assert_eq!(data, [0, 0x11, 1, 0, 0, 0, 0x11]);
    }

    #[test]
    fn store16() {
        let data = evaluate(
            [opcode::STORE | W16::FLAG],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0x22, 0x11, 0, 0, 0, 2],
        );
        assert_eq!(data, [0, 0, 0x11, 0x22, 2, 0, 0, 0, 0x11, 0x22]);
    }

    #[test]
    fn store32() {
        let data = evaluate(
            [opcode::STORE | W32::FLAG],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0x44, 0x33, 0x22, 0x11, 0, 0, 0, 4],
        );
        assert_eq!(
            data,
            [
                0, 0, 0, 0, 0x11, 0x22, 0x33, 0x44, 4, 0, 0, 0, 0x11, 0x22,
                0x33, 0x44
            ]
        );
    }

    #[test]
    fn store64() {
        let data = evaluate(
            [opcode::STORE | W64::FLAG],
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0,
            ],
            [0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0, 0, 0, 8],
        );
        assert_eq!(
            data,
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
                0x77, 0x88, 8, 0, 0, 0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
                0x77, 0x88
            ]
        );
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
            evaluator.push_u8(arg, &mut data);
        }

        evaluator.evaluate(&code, &mut data);
        data
    }
}
