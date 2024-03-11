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

    pub fn push_u8(&mut self, value: u8, data: &mut [u8]) {
        self.data.push([value], data);
    }

    pub fn push_u32(&mut self, value: u32, data: &mut [u8]) {
        self.data.push(value.to_le_bytes(), data);
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
                    let value = {
                        let mut bytes = [0; 1];
                        self.data.pop(&mut bytes, data);
                        bytes
                    };

                    self.data.store(address, value, data);
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
        evaluate([opcode::TERMINATE], [], []);
        // This should not run forever, nor cause any kind of panic.
    }

    #[test]
    fn push8() {
        let [a] = 0x11u8.to_le_bytes();
        let data = evaluate([opcode::PUSH | W8::FLAG, a], [0], []);
        assert_eq!(data, [0x11]);
    }

    #[test]
    fn push16() {
        let [a, b] = 0x2211u16.to_le_bytes();
        let data = evaluate([opcode::PUSH | W16::FLAG, a, b], [0, 0], []);
        assert_eq!(data, [0x11, 0x22]);
    }

    #[test]
    fn push32() {
        let [a, b, c, d] = 0x44332211u32.to_le_bytes();
        let data =
            evaluate([opcode::PUSH | W32::FLAG, a, b, c, d], [0, 0, 0, 0], []);
        assert_eq!(data, [0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn push64() {
        let [a, b, c, d, e, f, g, h] = 0x8877665544332211u64.to_le_bytes();
        let data = evaluate(
            [opcode::PUSH | W64::FLAG, a, b, c, d, e, f, g, h],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [],
        );
        assert_eq!(data, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
    }

    #[test]
    fn drop8() {
        let data =
            evaluate([opcode::DROP | W8::FLAG, opcode::PUSH, 255], [0], [127]);
        assert_eq!(data, [255]);
    }

    #[test]
    fn drop16() {
        let data = evaluate(
            [opcode::DROP | W16::FLAG, opcode::PUSH, 255],
            [0, 0],
            [127, 127],
        );
        assert_eq!(data, [127, 255]);
    }

    #[test]
    fn drop32() {
        let data = evaluate(
            [opcode::DROP | W32::FLAG, opcode::PUSH, 255],
            [0, 0, 0, 0],
            [127, 127, 127, 127],
        );
        assert_eq!(data, [127, 127, 127, 255]);
    }

    #[test]
    fn drop64() {
        let data = evaluate(
            [opcode::DROP | W64::FLAG, opcode::PUSH, 255],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [127, 127, 127, 127, 127, 127, 127, 127],
        );
        assert_eq!(data, [127, 127, 127, 127, 127, 127, 127, 255]);
    }

    #[test]
    fn store() {
        let data =
            evaluate([opcode::STORE], [0, 0, 0, 0, 0, 0], [255, 0, 0, 0, 0]);
        assert_eq!(data, [255, 0, 0, 0, 0, 255]);
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
