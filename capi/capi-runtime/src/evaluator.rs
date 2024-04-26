use crate::InstructionAddress;

use super::{
    builtins, code::Code, compiler::Instruction, data_stack::DataStack,
};

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Evaluator {
    pub code: Code,
    pub next_instruction: InstructionAddress,
    pub call_stack: Vec<InstructionAddress>,
    pub data_stack: DataStack,
}

impl Evaluator {
    pub fn update(&mut self, code: Code, entry: InstructionAddress) {
        self.code = code;
        self.next_instruction = entry;
    }

    pub fn step(&mut self, mem: &mut [u8]) -> EvaluatorState {
        let current_instruction = self.next_instruction;
        self.next_instruction.inc();

        let instruction = &self.code.instructions[current_instruction.0];

        match instruction {
            Instruction::CallBuiltin { name } => {
                let result = match name.as_str() {
                    "add" => builtins::add(&mut self.data_stack),
                    "copy" => builtins::copy(&mut self.data_stack),
                    "drop" => builtins::drop(&mut self.data_stack),
                    "mul" => builtins::mul(&mut self.data_stack),
                    "place" => builtins::place(&mut self.data_stack),
                    "sub" => builtins::sub(&mut self.data_stack),
                    "store" => builtins::store(&mut self.data_stack, mem),
                    "take" => builtins::take(&mut self.data_stack),
                    _ => panic!("Unknown builtin: `{name}`"),
                };

                if let Err(err) = result {
                    return EvaluatorState::Error {
                        err,
                        instruction: current_instruction,
                    };
                }
            }
            Instruction::CallFunction { name } => {
                let address = self.code.symbols.resolve(name);
                self.call_stack.push(self.next_instruction);
                self.next_instruction = address;
            }
            Instruction::PushValue(value) => self.data_stack.push(*value),
            Instruction::Return => {
                let Some(return_address) = self.call_stack.pop() else {
                    return EvaluatorState::Finished;
                };

                self.next_instruction = return_address;
            }
            Instruction::ReturnIfNonZero => {
                let a = self.data_stack.pop().unwrap();

                if a != 0 {
                    // Here we just duplicate the code from the regular return
                    // instruction above, which isn't great. Getting rid of the
                    // duplication completely isn't easy though, due to the
                    // `return`. And since I suspect that this instruction is
                    // temporary, until the language grows more features, I'm
                    // inclined to just leave this be.

                    let Some(return_address) = self.call_stack.pop() else {
                        return EvaluatorState::Finished;
                    };

                    self.next_instruction = return_address;
                }
            }
            Instruction::ReturnIfZero => {
                let a = self.data_stack.pop().unwrap();

                if a == 0 {
                    // Here we just duplicate the code from the regular return
                    // instruction above, which isn't great. Getting rid of the
                    // duplication completely isn't easy though, due to the
                    // `return`. And since I suspect that this instruction is
                    // temporary, until the language grows more features, I'm
                    // inclined to just leave this be.

                    let Some(return_address) = self.call_stack.pop() else {
                        return EvaluatorState::Finished;
                    };

                    self.next_instruction = return_address;
                }
            }
        }

        EvaluatorState::Running
    }
}

#[must_use]
pub enum EvaluatorState {
    Running,
    Finished,
    Error {
        err: builtins::Error,
        instruction: InstructionAddress,
    },
}
