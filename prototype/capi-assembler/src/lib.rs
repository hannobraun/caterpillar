use capi_vm::opcode;

pub fn assemble(assembly: &str) -> Option<Vec<u8>> {
    let mut bytecode = Vec::new();

    for instruction in assembly.split_whitespace() {
        match instruction {
            "terminate" => bytecode.push(opcode::TERMINATE),
            _ => return None,
        }
    }

    Some(bytecode)
}

#[cfg(test)]
mod tests {
    use capi_vm::Evaluator;

    #[test]
    fn terminate() {
        assemble("terminate");
        // This should not run forever, nor cause any kind of panic.
    }

    const DATA_SIZE: usize = 64;

    fn assemble(assembly: &str) -> [u8; DATA_SIZE] {
        let mut data = [0; DATA_SIZE];

        let bytecode = super::assemble(assembly).unwrap();

        let mut evaluator = Evaluator::new(&data);
        evaluator.evaluate(&bytecode, &mut data);

        data
    }
}
