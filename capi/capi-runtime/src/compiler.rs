use std::collections::BTreeSet;

use crate::{
    instructions::Instruction, source_map::SourceMap, syntax::Expression,
};

use super::{code::Code, syntax::ExpressionKind};

pub struct Compiler<'r> {
    pub functions: &'r BTreeSet<String>,
    pub code: &'r mut Code,
    pub source_map: &'r mut SourceMap,
}

impl Compiler<'_> {
    pub fn compile_function(&mut self, name: String, syntax: Vec<Expression>) {
        let address = self.code.next_address();

        for expression in syntax {
            compile_expression(expression, self);
        }

        self.code.push(Instruction::Return);
        self.code.symbols.define(name, address);
    }
}

fn compile_expression(expression: Expression, compiler: &mut Compiler) {
    let instruction = match expression.kind {
        ExpressionKind::Binding { .. } => {
            todo!("Compiling bindings is not supported yet.")
        }
        ExpressionKind::Comment { .. } => {
            return;
        }
        ExpressionKind::Value(value) => Instruction::Push { value },
        ExpressionKind::Word { name } => {
            word_to_instruction(name, compiler.functions)
        }
    };

    let address = compiler.code.push(instruction);
    compiler
        .source_map
        .define_mapping(address, expression.location)
}

fn word_to_instruction(
    word: String,
    functions: &BTreeSet<String>,
) -> Instruction {
    // Here we check for special built-in functions that are implemented
    // differently, without making sure anywhere, that its name doesn't conflict
    // with any user-defined functions.
    //
    // I think it's fine for now. This seems like a temporary hack anyway, while
    // the language is not powerful enough to support an actual `if`.
    if word == "return_if_non_zero" {
        return Instruction::ReturnIfNonZero;
    }
    if word == "return_if_zero" {
        return Instruction::ReturnIfZero;
    }

    // The code here would allow user-defined functions to shadow built-in
    // functions, which seems undesirable. It's better to catch this when
    // defining the function though, and while it would be nice to have a
    // fallback assertion here, that's not practical, given the way built-in
    // function resolution is implemented right now.
    if functions.contains(&word) {
        return Instruction::CallFunction { name: word };
    }

    // This doesn't check whether the built-in function exists, and given how
    // built-in functions are currently defined, it's not practical to
    // implement.
    Instruction::CallBuiltin { name: word }
}
