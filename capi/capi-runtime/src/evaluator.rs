use super::{
    builtins, code::Code, compiler::Instruction, data_stack::DataStack,
};

#[derive(Clone, Debug, Default)]
pub struct Evaluator {
    pub code: Code,
    pub instruction: usize,
    pub call_stack: Vec<usize>,
    pub data_stack: DataStack,
}

impl Evaluator {
    pub fn update(&mut self, code: Code, entry: usize) {
        self.code = code;
        self.instruction = entry;
    }

    pub fn step(&mut self, mem: &mut [u8]) -> bool {
        let instruction = &self.code.instructions[self.instruction];
        self.instruction += 1;

        match instruction {
            Instruction::CallBuiltin { name } => match name.as_str() {
                "add" => builtins::add(&mut self.data_stack),
                "copy" => builtins::copy(&mut self.data_stack),
                "drop" => builtins::drop(&mut self.data_stack),
                "mul" => builtins::mul(&mut self.data_stack),
                "place" => builtins::place(&mut self.data_stack),
                "sub" => builtins::sub(&mut self.data_stack),
                "store" => builtins::store(&mut self.data_stack, mem),
                "take" => builtins::take(&mut self.data_stack),
                _ => panic!("Unknown builtin: `{name}`"),
            },
            Instruction::CallFunction { name } => {
                let address = self.code.symbols.resolve(name);
                self.call_stack.push(self.instruction);
                self.instruction = address;
            }
            Instruction::PushValue(value) => self.data_stack.push(*value),
            Instruction::Return => {
                let Some(return_address) = self.call_stack.pop() else {
                    return false;
                };

                self.instruction = return_address;
            }
            Instruction::ReturnIfNonZero => {
                let a = self.data_stack.pop();

                if a != 0 {
                    // Here we just duplicate the code from the regular return
                    // instruction above, which isn't great. Getting rid of the
                    // duplication completely isn't easy though, due to the
                    // `return`. And since I suspect that this instruction is
                    // temporary, until the language grows more features, I'm
                    // inclined to just leave this be.

                    let Some(return_address) = self.call_stack.pop() else {
                        return false;
                    };

                    self.instruction = return_address;
                }
            }
            Instruction::ReturnIfZero => {
                let a = self.data_stack.pop();

                if a == 0 {
                    // Here we just duplicate the code from the regular return
                    // instruction above, which isn't great. Getting rid of the
                    // duplication completely isn't easy though, due to the
                    // `return`. And since I suspect that this instruction is
                    // temporary, until the language grows more features, I'm
                    // inclined to just leave this be.

                    let Some(return_address) = self.call_stack.pop() else {
                        return false;
                    };

                    self.instruction = return_address;
                }
            }
        }

        true
    }
}
