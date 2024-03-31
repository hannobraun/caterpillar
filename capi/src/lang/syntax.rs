use super::{compiler::Instruction, functions::Functions};

#[derive(Debug)]
pub struct Syntax<'r> {
    functions: &'r Functions,
    elements: &'r mut Vec<Instruction>,
}

impl<'r> Syntax<'r> {
    pub fn new(
        functions: &'r Functions,
        elements: &'r mut Vec<Instruction>,
    ) -> Self {
        Self {
            functions,
            elements,
        }
    }

    pub fn b(&mut self, name: &'static str) -> &mut Self {
        self.elements.push(Instruction::CallBuiltin { name });
        self
    }

    pub fn f(&mut self, name: &'static str) -> &mut Self {
        let address = self.functions.resolve(name);
        self.elements.push(Instruction::CallFunction { address });
        self
    }

    pub fn v(&mut self, value: usize) -> &mut Self {
        self.elements.push(Instruction::PushValue(value));
        self
    }
}
