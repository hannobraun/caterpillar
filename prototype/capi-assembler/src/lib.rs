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

        let mut opcode = String::new();
        let mut width = String::new();

        for ch in instruction.chars() {
            if ch.is_alphabetic() {
                if !width.is_empty() {
                    // We only have instructions that *end* with numbers, so
                    // this is definitely nothing we know about.
                    return Err(AssemblerError::UnknownInstruction {
                        name: instruction.to_owned(),
                    });
                }

                opcode.push(ch);
            } else if ch.is_ascii_digit() {
                width.push(ch);
            }
        }

        if opcode == "push" {
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
            "drop8" => Some((opcode::DROP, W8::INFO)),
            "drop16" => Some((opcode::DROP, W16::INFO)),
            "drop32" => Some((opcode::DROP, W32::INFO)),
            "drop64" => Some((opcode::DROP, W64::INFO)),
            "clone" => Some((opcode::CLONE, W8::INFO)),
            "store8" => Some((opcode::STORE, W8::INFO)),
            "store16" => Some((opcode::STORE, W16::INFO)),
            "store32" => Some((opcode::STORE, W32::INFO)),
            "store64" => Some((opcode::STORE, W64::INFO)),
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
    fn clone() -> anyhow::Result<()> {
        let data = assemble("push8 0x11 clone", [0; 2])?;
        assert_eq!(data, [0x11, 0x11]);
        Ok(())
    }

    #[test]
    fn drop8() -> anyhow::Result<()> {
        let data = assemble("push8 0x11 drop8 push8 0x22", [0])?;
        assert_eq!(data, [0x22]);
        Ok(())
    }

    #[test]
    fn drop16() -> anyhow::Result<()> {
        let data = assemble("push16 0x1111 drop16 push16 0x2222", [0, 0])?;
        assert_eq!(data, [0x22, 0x22]);
        Ok(())
    }

    #[test]
    fn drop32() -> anyhow::Result<()> {
        let data = assemble(
            "push32 0x11111111 drop32 push32 0x22222222",
            [0, 0, 0, 0],
        )?;
        assert_eq!(data, [0x22, 0x22, 0x22, 0x22]);
        Ok(())
    }

    #[test]
    fn drop64() -> anyhow::Result<()> {
        let data = assemble(
            "push64 0x111111111111111 drop64 push64 0x2222222222222222",
            [0, 0, 0, 0, 0, 0, 0, 0],
        )?;
        assert_eq!(data, [0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22]);
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
    fn store8() -> anyhow::Result<()> {
        let data = assemble("push8 0x11 push32 0 store8", [0, 0, 0, 0, 0, 0])?;
        assert_eq!(data, [0x11, 0, 0, 0, 0, 0x11]);
        Ok(())
    }

    #[test]
    fn store16() -> anyhow::Result<()> {
        let data = assemble(
            "push16 0x2211 push32 0 store16",
            [0, 0, 0, 0, 0, 0, 0, 0],
        )?;
        assert_eq!(data, [0x11, 0x22, 0, 0, 0, 0, 0x11, 0x22]);
        Ok(())
    }

    #[test]
    fn store32() -> anyhow::Result<()> {
        let data = assemble(
            "push32 0x44332211 push32 0 store32",
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )?;
        assert_eq!(
            data,
            [0x11, 0x22, 0x33, 0x44, 0, 0, 0, 0, 0x11, 0x22, 0x33, 0x44]
        );
        Ok(())
    }

    #[test]
    fn store64() -> anyhow::Result<()> {
        let data = assemble(
            "push64 0x8877665544332211 push32 0 store64",
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        )?;
        assert_eq!(
            data,
            [
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0, 0, 0, 0,
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88
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
