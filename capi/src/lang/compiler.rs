use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Compiler<'r> {
    pub functions: &'r Functions,
    pub instructions: Vec<Instruction>,
}

impl<'r> Compiler<'r> {
    pub fn new(functions: &'r Functions) -> Self {
        Self {
            functions,
            instructions: Vec::new(),
        }
    }

    pub fn b(&mut self, name: &'static str) -> &mut Self {
        self.instructions.push(Instruction::CallBuiltin { name });
        self
    }

    pub fn f(&mut self, name: &'static str) -> &mut Self {
        let Some(function) = self.functions.get(name) else {
            panic!("Could not resolve function `{name}`.");
        };
        self.instructions.extend(function.iter().copied());
        self
    }

    pub fn v(&mut self, value: usize) -> &mut Self {
        self.instructions.push(Instruction::PushValue(value));
        self
    }
}

pub type Functions = BTreeMap<&'static str, Vec<Instruction>>;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    CallBuiltin { name: &'static str },
    PushValue(usize),
    Return,
}
