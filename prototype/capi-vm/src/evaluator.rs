use crate::{code::Code, data::StoreError, opcode};

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

    pub fn push(&mut self, value: u32, data: &mut [u8]) -> &mut Self {
        self.data.push(value, data);
        self
    }

    pub fn pop(&mut self, data: &mut [u8]) -> u32 {
        self.data.pop(data)
    }

    pub fn evaluate(
        &mut self,
        code: impl AsRef<[u8]>,
        data: &mut [u8],
    ) -> Result<&mut Self, StoreError> {
        let code = code.as_ref();
        self.code.reset();

        loop {
            let Some(instruction) = self.code.read_instruction(code) else {
                break;
            };

            let opcode = instruction & 0x3f;

            match opcode {
                opcode::TERMINATE => {
                    break;
                }
                opcode::JUMP => {
                    let address = self.data.pop(data);
                    self.code.jump(address);
                }
                opcode::CALL => {
                    let address = self.data.pop(data);
                    let address = self.code.jump(address);
                    self.data.push(address, data);
                }
                opcode::PUSH => {
                    let value = self.code.read_value(code);
                    self.data.push(value, data);
                }
                opcode::DROP => {
                    let _ = self.data.pop(data);
                }
                opcode::STORE => {
                    let address = self.data.pop(data);
                    let value = self.data.pop(data);

                    self.data.store(address, value, data)?;
                }
                opcode::CLONE => {
                    let value = self.data.pop(data);

                    self.data.push(value, data);
                    self.data.push(value, data);
                }
                opcode::SWAP => {
                    let b = self.data.pop(data);
                    let a = self.data.pop(data);

                    self.data.push(b, data);
                    self.data.push(a, data);
                }
                opcode::AND => {
                    let b = self.data.pop(data);
                    let a = self.data.pop(data);

                    let r = a & b;

                    self.data.push(r, data);
                }
                opcode::OR => {
                    let b = self.data.pop(data);
                    let a = self.data.pop(data);

                    let r = a | b;

                    self.data.push(r, data);
                }
                opcode::ROL => {
                    let b = self.data.pop(data);
                    let a = self.data.pop(data);

                    let r = a.rotate_left(b);

                    self.data.push(r, data);
                }
                opcode => {
                    let opcode_as_char: char = opcode.into();
                    panic!("Unknown opcode: `{opcode_as_char}` ({opcode:#x})");
                }
            }
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::opcode::{
        AND, CALL, CLONE, DROP, JUMP, OR, PUSH, ROL, STORE, SWAP, TERMINATE,
    };

    use super::Evaluator;

    #[test]
    fn terminate() -> anyhow::Result<()> {
        let mut data = [];
        let mut evaluator = Evaluator::new(&data);

        evaluator.evaluate(bc().op(TERMINATE), &mut data)?;
        // This should not run forever, nor cause any kind of panic.

        Ok(())
    }

    #[test]
    fn jump() -> anyhow::Result<()> {
        let mut data = [0; 4];
        let mut evaluator = Evaluator::new(&data);

        evaluator.push(0x00000002, &mut data).evaluate(
            bc().op(JUMP).op(TERMINATE).op(PUSH).u32(0x44332211),
            &mut data,
        )?;

        assert_eq!(evaluator.pop(&mut data), 0x44332211);
        Ok(())
    }

    #[test]
    fn call() -> anyhow::Result<()> {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator.push(0x00000002, &mut data).evaluate(
            bc().op(CALL).op(TERMINATE).op(PUSH).u32(0x44332211),
            &mut data,
        )?;

        assert_eq!(evaluator.pop(&mut data), 0x44332211);
        assert_eq!(evaluator.pop(&mut data), 0x00000001);
        Ok(())
    }

    #[test]
    fn push() -> anyhow::Result<()> {
        let mut data = [0; 4];
        let mut evaluator = Evaluator::new(&data);

        evaluator.evaluate(bc().op(PUSH).u32(0x44332211), &mut data)?;

        assert_eq!(evaluator.pop(&mut data), 0x44332211);
        Ok(())
    }

    #[test]
    fn drop() -> anyhow::Result<()> {
        let mut data = [0; 4];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push(0x11111111, &mut data)
            .evaluate(bc().op(DROP), &mut data)?
            .push(0x22222222, &mut data);

        assert_eq!(evaluator.pop(&mut data), 0x22222222);
        Ok(())
    }

    #[test]
    fn store() -> anyhow::Result<()> {
        let mut data = [0; 16];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push(0x44332211, &mut data)
            .push(4, &mut data)
            .evaluate(bc().op(STORE), &mut data)?;

        assert_eq!(data[..8], [0, 0, 0, 0, 0x11, 0x22, 0x33, 0x44]);
        Ok(())
    }

    #[test]
    fn clone() -> anyhow::Result<()> {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push(0x11111111, &mut data)
            .evaluate(bc().op(CLONE), &mut data)?;

        assert_eq!(evaluator.pop(&mut data), 0x11111111);
        assert_eq!(evaluator.pop(&mut data), 0x11111111);
        Ok(())
    }

    #[test]
    fn swap() -> anyhow::Result<()> {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push(0x11111111, &mut data)
            .push(0x22222222, &mut data)
            .evaluate(bc().op(SWAP), &mut data)?;

        assert_eq!(evaluator.pop(&mut data), 0x11111111);
        assert_eq!(evaluator.pop(&mut data), 0x22222222);
        Ok(())
    }

    #[test]
    fn and() -> anyhow::Result<()> {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push(0x11111111, &mut data)
            .push(0x000000ff, &mut data)
            .evaluate(bc().op(AND), &mut data)?;

        assert_eq!(evaluator.pop(&mut data), 0x00000011);
        Ok(())
    }

    #[test]
    fn or() -> anyhow::Result<()> {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push(0x05030100, &mut data)
            .push(0x60402000, &mut data)
            .evaluate(bc().op(OR), &mut data)?;

        assert_eq!(evaluator.pop(&mut data), 0x65432100);
        Ok(())
    }

    #[test]
    fn rol() -> anyhow::Result<()> {
        let mut data = [0; 8];
        let mut evaluator = Evaluator::new(&data);

        evaluator
            .push(0x00ff00ff, &mut data)
            .push(8, &mut data)
            .evaluate(bc().op(ROL), &mut data)?;

        assert_eq!(evaluator.pop(&mut data), 0xff00ff00);
        Ok(())
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

        pub fn u32(mut self, value: u32) -> Self {
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
