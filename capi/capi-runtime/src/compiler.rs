use std::collections::BTreeSet;

use crate::{
    instructions::Instruction, source_map::SourceMap, syntax::Expression,
    SourceLocation,
};

use super::{code::Code, syntax::ExpressionKind};

pub struct Compiler<'r> {
    pub functions: &'r BTreeSet<String>,
    pub code: &'r mut Code,
    pub source_map: &'r mut SourceMap,
}

impl Compiler<'_> {
    pub fn compile_function(&mut self, name: String, syntax: Vec<Expression>) {
        let mut bindings = BTreeSet::new();
        let address = self.code.next_address();

        let mut last_location = None;
        for expression in syntax {
            last_location = Some(expression.location.clone());
            self.compile_expression(expression, &mut bindings);
        }

        self.generate(Instruction::Return, last_location);
        self.code.symbols.define(name, address);
    }

    fn compile_expression(
        &mut self,
        expression: Expression,
        bindings: &mut BTreeSet<String>,
    ) {
        match expression.kind {
            ExpressionKind::Binding { names } => {
                for name in names.into_iter().rev() {
                    // Inserting bindings unconditionally like that does mean
                    // that bindings can overwrite previously defined bindings.
                    // This is undesirable, but it'll do for now.
                    bindings.insert(name.clone());

                    self.generate(
                        Instruction::BindingDefine { name },
                        Some(expression.location.clone()),
                    );
                }
            }
            ExpressionKind::Comment { .. } => {}
            ExpressionKind::Value(value) => {
                self.generate(
                    Instruction::Push { value },
                    Some(expression.location),
                );
            }
            ExpressionKind::Word { name } => {
                let instruction =
                    word_to_instruction(name, bindings, self.functions);
                self.generate(instruction, Some(expression.location));
            }
        };
    }

    fn generate(
        &mut self,
        instruction: Instruction,
        location: Option<SourceLocation>,
    ) {
        let address = self.code.push(instruction);
        if let Some(location) = location {
            self.source_map.define_mapping(address, location);
        }
    }
}

fn word_to_instruction(
    word: String,
    bindings: &BTreeSet<String>,
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

    // The code here would allow bindings to shadow both user-defined and
    // builtin functions. This seems undesirable, without further handling to
    // prevent mistakes.
    //
    // It's better to catch this when defining bindings, though.
    if bindings.contains(&word) {
        return Instruction::BindingEvaluate { name: word };
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
