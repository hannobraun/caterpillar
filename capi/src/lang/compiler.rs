use std::collections::BTreeMap;

pub struct Compiler {
    pub functions: BTreeMap<&'static str, Vec<Instruction>>,
    pub fragments: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            functions: BTreeMap::new(),
            fragments: Vec::new(),
        }
    }

    pub fn b(&mut self, name: &'static str) -> &mut Self {
        self.fragments.push(Instruction::CallBuiltin { name });
        self
    }

    pub fn f(&mut self, name: &'static str) -> &mut Self {
        let function = self.functions.get(name).unwrap();
        self.fragments.extend(function.iter().copied());
        self
    }

    pub fn v(&mut self, value: usize) -> &mut Self {
        self.fragments.push(Instruction::Value(value));
        self
    }
}

#[derive(Copy, Clone)]
pub enum Instruction {
    CallBuiltin { name: &'static str },
    Value(usize),
}
