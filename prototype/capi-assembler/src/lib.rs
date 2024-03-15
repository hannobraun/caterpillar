use std::num::ParseIntError;

use capi_vm::{
    opcode,
    width::{Width, W32},
};

pub fn assemble(assembly: &str) -> Result<Vec<u8>, AssemblerError> {
    let mut bytecode = Vec::new();

    let mut instructions = assembly.split_whitespace();

    while let Some(instruction) = instructions.next() {
        if instruction.ends_with(':') {
            // This is a label. Currently they serve a function more like
            // comments, and are ignored.
            continue;
        }

        if instruction == "push" {
            let width = W32::INFO;

            let Some(value) = instructions.next() else {
                return Err(AssemblerError::PushCameLast);
            };

            let (value, radix) = match value.split_once("0x") {
                Some(("", value)) => (value, 16),
                None => (value, 10),

                Some((_, _)) => {
                    // We only have instructions with numbers at the end, so
                    // anything with a prefix before "0x" is definitely not an
                    // instruction we know.
                    return Err(AssemblerError::UnknownInstruction {
                        name: instruction.to_string(),
                    });
                }
            };

            let value = &u32::from_str_radix(value, radix)
                .map_err(|err| AssemblerError::ParseValue {
                    value: value.to_owned(),
                    source: err,
                })?
                .to_le_bytes();

            bytecode.push(opcode::PUSH | width.flag);
            for &b in value {
                bytecode.push(b);
            }

            continue;
        }

        let opcode_and_width = match instruction {
            "and" => Some(opcode::AND),
            "drop" => Some(opcode::DROP),
            "clone" => Some(opcode::CLONE),
            "or" => Some(opcode::OR),
            "rol" => Some(opcode::ROL),
            "store" => Some(opcode::STORE),
            "swap" => Some(opcode::SWAP),
            "terminate" => Some(opcode::TERMINATE),

            _ => None,
        };

        if let Some(opcode) = opcode_and_width {
            bytecode.push(opcode | W32::FLAG);
            continue;
        }

        return Err(AssemblerError::UnknownInstruction {
            name: instruction.into(),
        });
    }

    Ok(bytecode)
}

#[derive(Debug, thiserror::Error)]
pub enum AssemblerError {
    #[error("Could not parse value `{value}`")]
    ParseValue {
        value: String,
        source: ParseIntError,
    },

    #[error("Expected value after `push`, but came last")]
    PushCameLast,

    #[error("Unknown instruction: `{name}`")]
    UnknownInstruction { name: String },
}

#[cfg(test)]
mod tests {
    use capi_vm::Evaluator;

    use crate::AssemblerError;

    #[test]
    fn and() -> anyhow::Result<()> {
        let data = assemble("push 0x11111111 push 0x000000ff and", [0; 8])?;
        assert_eq!(data, [0xff, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00]);
        Ok(())
    }

    #[test]
    fn clone() -> anyhow::Result<()> {
        let data = assemble("push 0x11111111 clone", [0; 8])?;
        assert_eq!(data, [0x11; 8]);
        Ok(())
    }

    #[test]
    fn drop() -> anyhow::Result<()> {
        let data =
            assemble("push 0x11111111 drop push 0x22222222", [0, 0, 0, 0])?;
        assert_eq!(data, [0x22, 0x22, 0x22, 0x22]);
        Ok(())
    }

    #[test]
    fn or() -> anyhow::Result<()> {
        let data = assemble("push 0x05030100 push 0x60402000 or", [0; 8])?;
        assert_eq!(data, [0x00, 0x20, 0x40, 0x60, 0x00, 0x21, 0x43, 0x65]);
        Ok(())
    }

    #[test]
    fn push() -> anyhow::Result<()> {
        let data = assemble("push 0x44332211", [0, 0, 0, 0])?;
        assert_eq!(data, [0x11, 0x22, 0x33, 0x44]);
        Ok(())
    }

    #[test]
    fn push_decimal() -> anyhow::Result<()> {
        let data = assemble("push 1144201745", [0, 0, 0, 0])?;
        assert_eq!(data, [0x11, 0x22, 0x33, 0x44]);
        Ok(())
    }

    #[test]
    fn rol() -> anyhow::Result<()> {
        let data = assemble("push 0x00ff00ff push 8 rol", [0; 8])?;
        assert_eq!(data, [0x08, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0xff]);
        Ok(())
    }

    #[test]
    fn store() -> anyhow::Result<()> {
        let data = assemble(
            "push 0x44332211 push 0 store",
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )?;
        assert_eq!(
            data,
            [0x11, 0x22, 0x33, 0x44, 0, 0, 0, 0, 0x11, 0x22, 0x33, 0x44]
        );
        Ok(())
    }

    #[test]
    fn swap() -> anyhow::Result<()> {
        let data = assemble("push 0x11111111 push 0x22222222 swap", [0; 8])?;
        assert_eq!(data, [0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22]);
        Ok(())
    }

    #[test]
    fn terminate() -> anyhow::Result<()> {
        // This should not run forever, nor cause any kind of error.
        assemble("terminate", [])?;
        Ok(())
    }

    fn assemble<const D: usize>(
        assembly: &str,
        mut data: [u8; D],
    ) -> Result<[u8; D], AssemblerError> {
        let bytecode = super::assemble(assembly)?;

        let mut evaluator = Evaluator::new(&data);
        evaluator.evaluate(&bytecode, &mut data);

        Ok(data)
    }
}
