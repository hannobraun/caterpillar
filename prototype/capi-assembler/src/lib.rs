use std::num::ParseIntError;

use capi_vm::{
    opcode,
    width::{Width, W16, W32, W64, W8},
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

        if instruction.starts_with("push") {
            let width = match instruction {
                "push8" => W8::INFO,
                "push16" => W16::INFO,
                "push32" => W32::INFO,
                "push64" => W64::INFO,

                _ => {
                    return Err(AssemblerError::UnknownInstruction {
                        name: instruction.to_string(),
                    });
                }
            };

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

            let mut buffer = [0; 8];

            let mut parse = || -> Result<(), ParseIntError> {
                match width.flag {
                    W8::FLAG => {
                        buffer[..width.num_bytes].copy_from_slice(&[
                            u8::from_str_radix(value, radix)?,
                        ]);
                    }
                    W16::FLAG => {
                        buffer[..width.num_bytes].copy_from_slice(
                            &u16::from_str_radix(value, radix)?.to_le_bytes(),
                        );
                    }
                    W32::FLAG => {
                        buffer[..width.num_bytes].copy_from_slice(
                            &u32::from_str_radix(value, radix)?.to_le_bytes(),
                        );
                    }
                    W64::FLAG => {
                        buffer[..width.num_bytes].copy_from_slice(
                            &u64::from_str_radix(value, radix)?.to_le_bytes(),
                        );
                    }
                    _ => {
                        unreachable!("Unsupported width");
                    }
                }

                Ok(())
            };

            parse().map_err(|err| AssemblerError::ParseValue {
                value: value.to_owned(),
                source: err,
            })?;
            let value = &buffer[..width.num_bytes];

            bytecode.push(opcode::PUSH | width.flag);
            for &b in value {
                bytecode.push(b);
            }

            continue;
        }

        let opcode_and_width = match instruction {
            "drop" => Some((opcode::DROP, W32::INFO)),
            "clone8" => Some((opcode::CLONE, W8::INFO)),
            "clone16" => Some((opcode::CLONE, W16::INFO)),
            "clone32" => Some((opcode::CLONE, W32::INFO)),
            "clone64" => Some((opcode::CLONE, W64::INFO)),
            "store" => Some((opcode::STORE, W32::INFO)),
            "swap8" => Some((opcode::SWAP, W8::INFO)),
            "swap16" => Some((opcode::SWAP, W16::INFO)),
            "swap32" => Some((opcode::SWAP, W32::INFO)),
            "swap64" => Some((opcode::SWAP, W64::INFO)),
            "terminate" => Some((opcode::TERMINATE, W8::INFO)),

            _ => None,
        };

        if let Some((opcode, width)) = opcode_and_width {
            bytecode.push(opcode | width.flag);
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
    fn clone8() -> anyhow::Result<()> {
        let data = assemble("push8 0x11 clone8", [0; 2])?;
        assert_eq!(data, [0x11; 2]);
        Ok(())
    }

    #[test]
    fn clone16() -> anyhow::Result<()> {
        let data = assemble("push16 0x1111 clone16", [0; 4])?;
        assert_eq!(data, [0x11; 4]);
        Ok(())
    }

    #[test]
    fn clone32() -> anyhow::Result<()> {
        let data = assemble("push32 0x11111111 clone32", [0; 8])?;
        assert_eq!(data, [0x11; 8]);
        Ok(())
    }

    #[test]
    fn clone64() -> anyhow::Result<()> {
        let data = assemble("push64 0x1111111111111111 clone64", [0; 16])?;
        assert_eq!(data, [0x11; 16]);
        Ok(())
    }

    #[test]
    fn drop32() -> anyhow::Result<()> {
        let data =
            assemble("push32 0x11111111 drop push32 0x22222222", [0, 0, 0, 0])?;
        assert_eq!(data, [0x22, 0x22, 0x22, 0x22]);
        Ok(())
    }

    #[test]
    fn push8_decimal() -> anyhow::Result<()> {
        let data = assemble("push8 255", [0])?;
        assert_eq!(data, [255]);
        Ok(())
    }

    #[test]
    fn push8() -> anyhow::Result<()> {
        let data = assemble("push8 0x11", [0])?;
        assert_eq!(data, [0x11]);
        Ok(())
    }

    #[test]
    fn push16() -> anyhow::Result<()> {
        let data = assemble("push16 0x2211", [0, 0])?;
        assert_eq!(data, [0x11, 0x22]);
        Ok(())
    }

    #[test]
    fn push32() -> anyhow::Result<()> {
        let data = assemble("push32 0x44332211", [0, 0, 0, 0])?;
        assert_eq!(data, [0x11, 0x22, 0x33, 0x44]);
        Ok(())
    }

    #[test]
    fn push64() -> anyhow::Result<()> {
        let data =
            assemble("push64 0x8877665544332211", [0, 0, 0, 0, 0, 0, 0, 0])?;
        assert_eq!(data, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
        Ok(())
    }

    #[test]
    fn store() -> anyhow::Result<()> {
        let data = assemble(
            "push32 0x44332211 push32 0 store",
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )?;
        assert_eq!(
            data,
            [0x11, 0x22, 0x33, 0x44, 0, 0, 0, 0, 0x11, 0x22, 0x33, 0x44]
        );
        Ok(())
    }

    #[test]
    fn swap8() -> anyhow::Result<()> {
        let data = assemble("push8 0x11 push8 0x22 swap8", [0; 2])?;
        assert_eq!(data, [0x11, 0x22]);
        Ok(())
    }

    #[test]
    fn swap16() -> anyhow::Result<()> {
        let data = assemble("push16 0x1111 push16 0x2222 swap16", [0; 4])?;
        assert_eq!(data, [0x11, 0x11, 0x22, 0x22]);
        Ok(())
    }

    #[test]
    fn swap32() -> anyhow::Result<()> {
        let data =
            assemble("push32 0x11111111 push32 0x22222222 swap32", [0; 8])?;
        assert_eq!(data, [0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22]);
        Ok(())
    }

    #[test]
    fn swap64() -> anyhow::Result<()> {
        let data = assemble(
            "push64 0x1111111111111111 push64 0x2222222222222222 swap64",
            [0; 16],
        )?;
        assert_eq!(
            data,
            [
                0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x22, 0x22,
                0x22, 0x22, 0x22, 0x22, 0x22, 0x22
            ]
        );
        Ok(())
    }

    #[test]
    fn terminate() -> anyhow::Result<()> {
        // This should not run forever, nor cause any kind of error.
        assemble("terminate", [])?;
        Ok(())
    }

    #[test]
    fn unknown_mixed_alphanumerics() {
        let result = assemble("pu32sh", []);
        assert!(matches!(
            result,
            Err(AssemblerError::UnknownInstruction { .. })
        ));
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
