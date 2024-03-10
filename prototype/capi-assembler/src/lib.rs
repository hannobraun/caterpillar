use std::num::ParseIntError;

use capi_vm::opcode;

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

        for ch in instruction.chars() {
            if ch.is_alphabetic() {
                opcode.push(ch);
            }
        }

        if opcode == "push" {
            let Some(value) = instructions.next() else {
                return Err(AssemblerError::PushCameLast);
            };

            let radix = 10;
            let value = u8::from_str_radix(value, radix)?;

            bytecode.push(opcode::PUSH);
            bytecode.push(value);

            continue;
        }

        let opcode = match opcode.as_str() {
            "clone" => Some(opcode::CLONE),
            "drop" => Some(opcode::DROP),
            "store" => Some(opcode::STORE),
            "terminate" => Some(opcode::TERMINATE),
            _ => None,
        };

        if let Some(opcode) = opcode {
            bytecode.push(opcode);
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
    #[error("Could not parse value")]
    ParseValue(#[from] ParseIntError),

    #[error("Expected value after `push`, but came last")]
    PushCameLast,

    #[error("Unknown instruction: `{name}`")]
    UnknownInstruction { name: String },
}

#[cfg(test)]
mod tests {
    use capi_vm::Evaluator;

    #[test]
    fn clone() {
        let data = assemble("push8 255 clone", [0, 0]);
        assert_eq!(data, [255, 255]);
    }

    #[test]
    fn drop() {
        let data = assemble("push8 255 drop push8 127", [0]);
        assert_eq!(data, [127]);
    }

    #[test]
    fn push() {
        let data = assemble("push8 255", [0]);
        assert_eq!(data, [255]);
    }

    #[test]
    fn store() {
        let data = assemble("push8 255 push8 0 store", [0, 0, 0]);
        assert_eq!(data, [255, 0, 255]);
    }

    #[test]
    fn terminate() {
        assemble("terminate", []);
        // This should not run forever, nor cause any kind of panic.
    }

    fn assemble<const D: usize>(assembly: &str, mut data: [u8; D]) -> [u8; D] {
        let bytecode = super::assemble(assembly).unwrap();

        let mut evaluator = Evaluator::new(&data);
        evaluator.evaluate(&bytecode, &mut data);

        data
    }
}
