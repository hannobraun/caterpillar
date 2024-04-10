use super::{
    builtins, code::Code, compiler::Instruction, data_stack::DataStack,
};

#[derive(Debug)]
pub struct Evaluator {
    pub code: Code,
    pub call_stack: Vec<usize>,
    pub data_stack: DataStack,
}

impl Evaluator {
    pub fn new(code: Code) -> Self {
        Self {
            code,
            call_stack: Vec::new(),
            data_stack: DataStack::new(),
        }
    }

    pub fn evaluate(&mut self, entry: usize, mem: &mut [u8]) {
        let mut current_instruction = entry;

        loop {
            let instruction = &self.code.instructions[current_instruction];
            current_instruction += 1;

            match instruction {
                Instruction::CallBuiltin { name } => match name.as_str() {
                    "add" => builtins::add(&mut self.data_stack),
                    "clone2" => builtins::clone2(&mut self.data_stack),
                    "drop2" => builtins::drop2(&mut self.data_stack),
                    "mul" => builtins::mul(&mut self.data_stack),
                    "pick" => builtins::pick(&mut self.data_stack),
                    "sub" => builtins::sub(&mut self.data_stack),
                    "store" => builtins::store(&mut self.data_stack, mem),
                    _ => panic!("Unknown builtin: `{name}`"),
                },
                Instruction::CallFunction { name } => {
                    let address = self.code.symbols.resolve(name);
                    self.call_stack.push(current_instruction);
                    current_instruction = address;
                }
                Instruction::PushValue(value) => self.data_stack.push(*value),
                Instruction::Return => {
                    let Some(return_address) = self.call_stack.pop() else {
                        break;
                    };

                    current_instruction = return_address;
                }
                Instruction::ReturnIfZero => {
                    let value = self.data_stack.pop();

                    if value == 0 {
                        // Here we just duplicate the code from the regular
                        // return instruction above, which isn't great. Getting
                        // rid of the duplication completely isn't easy though,
                        // due to the `break`, and since I suspect that this
                        // instruction is temporary, until the language grows
                        // more features, I'm inclined to just leave this be.

                        let Some(return_address) = self.call_stack.pop() else {
                            break;
                        };

                        current_instruction = return_address;
                    }
                }
            }
        }
    }
}
