use super::functions::Functions;

#[derive(Debug)]
pub struct Compiler<'r> {
    functions: &'r Functions,
    instructions: &'r mut Vec<Instruction>,
}

impl<'r> Compiler<'r> {
    pub fn new(
        functions: &'r Functions,
        instructions: &'r mut Vec<Instruction>,
    ) -> Self {
        Self {
            functions,
            instructions,
        }
    }

    pub fn b(&mut self, name: &'static str) -> &mut Self {
        self.instructions.push(Instruction::CallBuiltin { name });
        self
    }

    pub fn f(&mut self, name: &'static str) -> &mut Self {
        let address = self.functions.resolve(name);
        self.instructions
            .push(Instruction::CallFunction { address });
        self
    }

    pub fn v(&mut self, value: usize) -> &mut Self {
        self.instructions.push(Instruction::PushValue(value));
        self
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    CallBuiltin { name: &'static str },
    CallFunction { address: usize },
    PushValue(usize),
    Return,
}
