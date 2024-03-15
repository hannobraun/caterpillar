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
        self.code.ptr = 0;

        loop {
            let Some(&instruction) = code.get(self.code.ptr) else {
                break;
            };
            self.code.ptr += 1;

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
                    let mut buffer = [0; MAX_WIDTH_BYTES];
                    let value = &mut buffer[0..width.num_bytes];

                    for b in value {
                        *b = code[self.code.ptr];
                        self.code.ptr += 1;
                    }

                    self.data
                        .push(buffer.into_iter().take(width.num_bytes), data);
                }
                opcode::DROP => {
                    let mut buffer = [0; MAX_WIDTH_BYTES];
                    let value = &mut buffer[..width.num_bytes];
                    self.data.pop(value, data);
                }
                opcode::STORE => {
                    let address = {
                        let mut bytes = [0; 4];
                        self.data.pop(&mut bytes, data);

                        u32::from_le_bytes(bytes)
                    };

                    let mut buffer = [0; MAX_WIDTH_BYTES];
                    let value = &mut buffer[..width.num_bytes];
                    self.data.pop(value, data);

                    self.data.store(address, value.iter().copied(), data);
                }
                opcode::CLONE => {
                    let mut value = [0; MAX_WIDTH_BYTES];

                    self.data.pop(&mut value[..width.num_bytes], data);

                    self.data
                        .push(value.into_iter().take(width.num_bytes), data);
                    self.data
                        .push(value.into_iter().take(width.num_bytes), data);
                }
                opcode::SWAP => {
                    let mut a = [0; MAX_WIDTH_BYTES];
                    let mut b = [0; MAX_WIDTH_BYTES];

                    self.data.pop(&mut a[..width.num_bytes], data);
                    self.data.pop(&mut b[..width.num_bytes], data);

                    self.data.push(a.into_iter().take(width.num_bytes), data);
                    self.data.push(b.into_iter().take(width.num_bytes), data);
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
        opcode::{CLONE, DROP, PUSH, STORE, SWAP, TERMINATE},
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
    fn drop8() {
        let mut data = [0; 2];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u8(0x11, &mut data)
            .push_u8(0x22, &mut data)
            .evaluate(bc().op(DROP).w(W8).op(PUSH).u8(0x33), &mut data);

        assert_eq!(data, [0x33, 0x11]);
    }

    #[test]
    fn drop16() {
        let mut data = [0; 4];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u16(0x1111, &mut data)
            .push_u16(0x2222, &mut data)
            .evaluate(bc().op(DROP).w(W16).op(PUSH).u8(0x33), &mut data);

        assert_eq!(data, [0x22, 0x33, 0x11, 0x11]);
    }

    #[test]
    fn drop32() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u32(0x11111111, &mut data)
            .push_u32(0x22222222, &mut data)
            .evaluate(bc().op(DROP).w(W32).op(PUSH).u8(0x33), &mut data);

        assert_eq!(data, [0x22, 0x22, 0x22, 0x33, 0x11, 0x11, 0x11, 0x11]);
    }

    #[test]
    fn drop64() {
        let mut data = [0; 16];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u64(0x1111111111111111, &mut data)
            .push_u64(0x2222222222222222, &mut data)
            .evaluate(bc().op(DROP).w(W64).op(PUSH).u8(0x33), &mut data);

        assert_eq!(
            data,
            [
                0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x33, 0x11, 0x11,
                0x11, 0x11, 0x11, 0x11, 0x11, 0x11
            ]
        );
    }

    #[test]
    fn store8() {
        let mut data = [0; 7];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u8(0x11, &mut data)
            .push_u32(1, &mut data)
            .evaluate(bc().op(STORE).w(W8), &mut data);

        assert_eq!(data, [0, 0x11, 1, 0, 0, 0, 0x11]);
    }

    #[test]
    fn store16() {
        let mut data = [0; 10];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u16(0x2211, &mut data)
            .push_u32(2, &mut data)
            .evaluate(bc().op(STORE).w(W16), &mut data);

        assert_eq!(data, [0, 0, 0x11, 0x22, 2, 0, 0, 0, 0x11, 0x22]);
    }

    #[test]
    fn store32() {
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
    fn store64() {
        let mut data = [0; 28];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u64(0x8877665544332211, &mut data)
            .push_u32(8, &mut data)
            .evaluate(bc().op(STORE).w(W64), &mut data);

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
    fn clone8() {
        let mut data = [0; 2];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u8(0x11, &mut data)
            .evaluate(bc().op(CLONE).w(W8), &mut data);

        assert_eq!(data, [0x11, 0x11]);
    }

    #[test]
    fn clone16() {
        let mut data = [0; 4];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u16(0x1111, &mut data)
            .evaluate(bc().op(CLONE).w(W16), &mut data);

        assert_eq!(data, [0x11, 0x11, 0x11, 0x11]);
    }

    #[test]
    fn clone32() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u32(0x11111111, &mut data)
            .evaluate(bc().op(CLONE).w(W32), &mut data);

        assert_eq!(data, [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11]);
    }

    #[test]
    fn clone64() {
        let mut data = [0; 16];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u64(0x1111111111111111, &mut data)
            .evaluate(bc().op(CLONE).w(W64), &mut data);

        assert_eq!(
            data,
            [
                0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                0x11, 0x11, 0x11, 0x11, 0x11, 0x11
            ]
        );
    }

    #[test]
    fn swap8() {
        let mut data = [0; 2];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u8(0x11, &mut data)
            .push_u8(0x22, &mut data)
            .evaluate(bc().op(SWAP).w(W8), &mut data);

        assert_eq!(data, [0x11, 0x22]);
    }

    #[test]
    fn swap16() {
        let mut data = [0; 4];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u16(0x1111, &mut data)
            .push_u16(0x2222, &mut data)
            .evaluate(bc().op(SWAP).w(W16), &mut data);

        assert_eq!(data, [0x11, 0x11, 0x22, 0x22]);
    }

    #[test]
    fn swap32() {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u32(0x11111111, &mut data)
            .push_u32(0x22222222, &mut data)
            .evaluate(bc().op(SWAP).w(W32), &mut data);

        assert_eq!(data, [0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22]);
    }

    #[test]
    fn swap64() {
        let mut data = [0; 16];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push_u64(0x1111111111111111, &mut data)
            .push_u64(0x2222222222222222, &mut data)
            .evaluate(bc().op(SWAP).w(W64), &mut data);

        assert_eq!(
            data,
            [
                0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22,
                0x22, 0x22, 0x22, 0x22, 0x22, 0x22
            ]
        );
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
