use std::{collections::BTreeMap, num::ParseIntError};

use capi_vm::opcode;

pub fn assemble(assembly: &str) -> Result<Vec<u8>, AssemblerError> {
    let mut bytecode = Vec::new();

    let mut labels = BTreeMap::new();
    let mut references = BTreeMap::new();

    let mut instructions = assembly.split_whitespace();

    while let Some(instruction) = instructions.next() {
        if let Some((label, "")) = instruction.split_once(':') {
            let address: u32 = bytecode
                .len()
                .try_into()
                .expect("Failed to convert `usize` to `u32`");
            labels.insert(label, address);
            continue;
        }

        if instruction == "push" {
            bytecode.push(opcode::PUSH);

            let Some(value) = instructions.next() else {
                return Err(AssemblerError::PushCameLast);
            };

            if let Some(("", reference)) = value.split_once(':') {
                references.insert(reference, bytecode.len());
                bytecode.extend([0; 4]);
                continue;
            }

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

            let value = u32::from_str_radix(value, radix).map_err(|err| {
                AssemblerError::ParseValue {
                    value: value.to_owned(),
                    source: err,
                }
            })?;

            for b in value.to_le_bytes() {
                bytecode.push(b);
            }

            continue;
        }

        let opcode_and_width = match instruction {
            "and" => Some(opcode::AND),
            "call" => Some(opcode::CALL),
            "drop" => Some(opcode::DROP),
            "clone" => Some(opcode::CLONE),
            "jump" => Some(opcode::JUMP),
            "or" => Some(opcode::OR),
            "rol" => Some(opcode::ROL),
            "store" => Some(opcode::STORE),
            "swap" => Some(opcode::SWAP),
            "terminate" => Some(opcode::TERMINATE),

            _ => None,
        };

        if let Some(opcode) = opcode_and_width {
            bytecode.push(opcode);
            continue;
        }

        return Err(AssemblerError::UnknownInstruction {
            name: instruction.into(),
        });
    }

    for (reference, position) in references {
        let Some(address) = labels.get(reference) else {
            return Err(AssemblerError::UnknownLabel {
                name: reference.into(),
            });
        };

        bytecode[position..position + 4]
            .copy_from_slice(&address.to_le_bytes());
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

    #[error("Unknown label: `{name}`")]
    UnknownLabel { name: String },
}

#[cfg(test)]
mod tests {
    use capi_vm::Evaluator;

    #[test]
    fn and() -> anyhow::Result<()> {
        let data = assemble("push 0x11111111 push 0x000000ff and", [0; 8])?;
        assert_eq!(data, [0xff, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00]);
        Ok(())
    }

    #[test]
    fn call() -> anyhow::Result<()> {
        let data = assemble("push 7 call terminate push 0x11111111", [0; 8])?;
        assert_eq!(data, [0x11, 0x11, 0x11, 0x11, 0x06, 0x00, 0x00, 0x00]);
        Ok(())
    }

    #[test]
    fn call_label() -> anyhow::Result<()> {
        let data = assemble(
            "
            start:
                push :push
                call
                terminate
            push:
                push 0x11111111
            ",
            [0; 8],
        )?;
        assert_eq!(data, [0x11, 0x11, 0x11, 0x11, 0x06, 0x00, 0x00, 0x00]);
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
    fn jump() -> anyhow::Result<()> {
        let data = assemble("push 7 jump terminate push 0x11111111", [0; 4])?;
        assert_eq!(data, [0x11, 0x11, 0x11, 0x11]);
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
    ) -> anyhow::Result<[u8; D]> {
        let bytecode = super::assemble(assembly)?;

        let mut evaluator = Evaluator::new(&data);
        evaluator.evaluate(&bytecode, &mut data);

        Ok(data)
    }
}
