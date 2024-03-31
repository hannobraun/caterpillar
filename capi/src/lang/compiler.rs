use super::{functions::Functions, syntax::SyntaxElement};

pub fn compile(
    syntax: Vec<SyntaxElement>,
    _: &Functions,
    instructions: &mut Vec<Instruction>,
) {
    instructions.extend(syntax.into_iter().map(|syntax_element| {
        match syntax_element {
            SyntaxElement::Word { name } => Instruction::CallBuiltin { name },
            SyntaxElement::CallFunction { address } => {
                Instruction::CallFunction { address }
            }
            SyntaxElement::Value(value) => Instruction::PushValue(value),
        }
    }));
    instructions.push(Instruction::Return);
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    CallBuiltin { name: &'static str },
    CallFunction { address: usize },
    PushValue(usize),
    Return,
}
