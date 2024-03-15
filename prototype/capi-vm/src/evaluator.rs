use crate::{
    code::Code,
    opcode,
    width::{Width, MAX_WIDTH_BYTES, W16, W32, W64, W8},
};

use super::data::Data;

pub struct Evaluator {
    code: Code,
    data: Data,
}

impl Evaluator {
    pub fn new(data: &[u8]) -> Self {
        Self {
            code: Code::new(),
            data: Data::new(data),
        }
    }

    pub fn push_i8(&mut self, value: i8, data: &mut [u8]) -> &mut Self {
        self.data.push(value.to_le_bytes(), data);
        self
    }

    pub fn push_u8(&mut self, value: u8, data: &mut [u8]) -> &mut Self {
        self.data.push(value.to_le_bytes(), data);
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

    pub fn evaluate(&mut self, code: impl AsRef<[u8]>, data: &mut [u8]) {
        let code = code.as_ref();
        self.code.reset();

        loop {
            let Some(instruction) = self.code.read_instruction(code) else {
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
                opcode::JUMP => {
                    let mut offset = [0; 1];
                    let _ = self.data.pop(&mut offset, data);

                    let offset = i8::from_le_bytes(offset).into();
                    self.code.jump_relative(offset);
                }
                opcode::PUSH => {
                    let mut buffer = [0; MAX_WIDTH_BYTES];
                    let value = self
                        .code
                        .read_value(&mut buffer[0..width.num_bytes], code);

                    self.data.push(value, data);
                }
                opcode::DROP => {
                    let mut buffer = [0; 4];
                    let _ = self.data.pop(&mut buffer, data);
                }
                opcode::STORE => {
                    let address = {
                        let mut bytes = [0; 4];
                        let _ = self.data.pop(&mut bytes, data);

                        u32::from_le_bytes(bytes)
                    };

                    let mut buffer = [0; 4];
                    let value = self.data.pop(&mut buffer, data);

                    self.data.store(address, value, data);
                }
                opcode::CLONE => {
                    let mut value = [0; 4];

                    let value = self.data.pop(&mut value, data);

                    self.data.push(value.clone(), data);
                    self.data.push(value, data);
                }
                opcode::SWAP => {
                    let mut b = [0; 4];
                    let mut a = [0; 4];

                    let b = self.data.pop(&mut b, data);
                    let a = self.data.pop(&mut a, data);

                    self.data.push(b, data);
                    self.data.push(a, data);
                }
                opcode::AND => {
                    let mut b = [0; 4];
                    let mut a = [0; 4];

                    let _ = self.data.pop(&mut b, data);
                    let _ = self.data.pop(&mut a, data);

                    let b = u32::from_le_bytes(b);
                    let a = u32::from_le_bytes(a);

                    let r = a & b;

                    self.data.push(r.to_le_bytes(), data);
                }
                opcode::OR => {
                    let mut b = [0; 4];
                    let mut a = [0; 4];

                    let _ = self.data.pop(&mut b, data);
                    let _ = self.data.pop(&mut a, data);

                    let b = u32::from_le_bytes(b);
                    let a = u32::from_le_bytes(a);

                    let r = a | b;

                    self.data.push(r.to_le_bytes(), data);
                }
                opcode::ROL => {
                    let mut b = [0; 4];
                    let mut a = [0; 4];

                    let _ = self.data.pop(&mut b, data);
                    let _ = self.data.pop(&mut a, data);

                    let b = u32::from_le_bytes(b);
                    let a = u32::from_le_bytes(a);

                    let r = a.rotate_left(b);

                    self.data.push(r.to_le_bytes(), data);
                }
                opcode => {
                    let opcode_as_char: char = opcode.into();
                    panic!("Unknown opcode: `{opcode_as_char}` ({opcode:#x})");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        opcode::{
            AND, CLONE, DROP, JUMP, OR, PUSH, ROL, STORE, SWAP, TERMINATE,
        },
        width::{Width, W16, W32, W64, W8},
    };

    use super::Evaluator;

    #[test]
    fn terminate() {
        let mut data = [];
        let mut evaluator = Evaluator::new(&data);

        evaluator.evaluate(bc().op(TERMINATE), &mut data);
        // This should not run forever, nor cause any kind of panic.
    }

    #[test]
    fn jump8r_simple() {
        let mut data = [0; 1];
        let mut evaluator = Evaluator::new(&data);

        evaluator.push_i8(1, &mut data).evaluate(
            bc().op(JUMP).op(TERMINATE).op(PUSH).w(W8).u8(0x11),
            &mut data,
        );
        assert_eq!(data, [0x11]);
    }

    #[test]
    fn jump8r_negative() {
        let mut data = [0; 3];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_i8(2, &mut data)
            .push_i8(-2, &mut data)
            .push_i8(1, &mut data)
            .evaluate(
                bc().op(JUMP)
                    .op(JUMP)
                    .op(JUMP)
                    .op(TERMINATE)
                    .op(PUSH)
                    .w(W8)
                    .u8(0x11),
                &mut data,
            );
        assert_eq!(data, [1, -2i8 as u8, 0x11]);
    }

    #[test]
    fn push8() {
        let mut data = [0; 1];
        let mut evaluator = Evaluator::new(&data);

        evaluator.evaluate(bc().op(PUSH).w(W8).u8(0x11), &mut data);

        assert_eq!(data, [0x11]);
    }

    #[test]
    fn push16() {
        let mut data = [0; 2];
        let mut evaluator = Evaluator::new(&data);

        evaluator.evaluate(bc().op(PUSH).w(W16).u16(0x2211), &mut data);

        assert_eq!(data, [0x11, 0x22]);
    }

    #[test]
    fn push32() {
        let mut data = [0; 4];
        let mut evaluator = Evaluator::new(&data);

        evaluator.evaluate(bc().op(PUSH).w(W32).u32(0x44332211), &mut data);

        assert_eq!(data, [0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn push64() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .evaluate(bc().op(PUSH).w(W64).u64(0x8877665544332211), &mut data);

        assert_eq!(data, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
    }

    #[test]
    fn drop() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u32(0x11111111, &mut data)
            .push_u32(0x22222222, &mut data)
            .evaluate(
                bc().op(DROP).w(W32).op(PUSH).w(W32).u32(0x33333333),
                &mut data,
            );

        assert_eq!(data, [0x33, 0x33, 0x33, 0x33, 0x11, 0x11, 0x11, 0x11]);
    }

    #[test]
    fn store() {
        let mut data = [0; 16];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u32(0x44332211, &mut data)
            .push_u32(4, &mut data)
            .evaluate(bc().op(STORE).w(W32), &mut data);

        assert_eq!(
            data,
            [
                0, 0, 0, 0, 0x11, 0x22, 0x33, 0x44, 4, 0, 0, 0, 0x11, 0x22,
                0x33, 0x44
            ]
        );
    }

    #[test]
    fn clone() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u32(0x11111111, &mut data)
            .evaluate(bc().op(CLONE).w(W32), &mut data);

        assert_eq!(data, [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11]);
    }

    #[test]
    fn swap() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u32(0x11111111, &mut data)
            .push_u32(0x22222222, &mut data)
            .evaluate(bc().op(SWAP).w(W32), &mut data);

        assert_eq!(data, [0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22]);
    }

    #[test]
    fn and() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u32(0x11111111, &mut data)
            .push_u32(0x000000ff, &mut data)
            .evaluate(bc().op(AND), &mut data);

        assert_eq!(data, [0xff, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn or() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u32(0x05030100, &mut data)
            .push_u32(0x60402000, &mut data)
            .evaluate(bc().op(OR), &mut data);

        assert_eq!(data, [0x00, 0x20, 0x40, 0x60, 0x00, 0x21, 0x43, 0x65]);
    }

    #[test]
    fn rol() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u32(0x00ff00ff, &mut data)
            .push_u32(8, &mut data)
            .evaluate(bc().op(ROL), &mut data);

        assert_eq!(data, [0x08, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0xff]);
    }

    pub fn bc() -> Bytecode {
        Bytecode { inner: Vec::new() }
    }

    pub struct Bytecode {
        inner: Vec<u8>,
    }

    impl Bytecode {
        pub fn op(mut self, opcode: u8) -> Self {
            self.inner.push(opcode);
            self
        }

        pub fn w<W>(mut self, _: W) -> Self
        where
            W: Width,
        {
            *self
                .inner
                .last_mut()
                .expect("Expected previous call to `op`") |= W::FLAG;
            self
        }

        pub fn u8(mut self, value: u8) -> Self {
            self.inner.extend(value.to_le_bytes());
            self
        }

        pub fn u16(mut self, value: u16) -> Self {
            self.inner.extend(value.to_le_bytes());
            self
        }

        pub fn u32(mut self, value: u32) -> Self {
            self.inner.extend(value.to_le_bytes());
            self
        }

        pub fn u64(mut self, value: u64) -> Self {
            self.inner.extend(value.to_le_bytes());
            self
        }
    }

    impl AsRef<[u8]> for Bytecode {
        fn as_ref(&self) -> &[u8] {
            self.inner.as_ref()
        }
    }
}
