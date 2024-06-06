use std::collections::BTreeSet;

use crate::{
    instructions::Instruction,
    runtime,
    source_map::SourceMap,
    syntax::{Expression, Function, Location, Script},
    Program,
};

use super::{code::Code, syntax::ExpressionKind};

pub fn compile(script: Script, entry: &str) -> Program {
    let mut code = Code::new();
    let mut source_map = SourceMap::default();

    let mut compiler = Compiler {
        functions: &script.functions.names,
        code: &mut code,
        source_map: &mut source_map,
    };

    for Function { name, args, syntax } in &script.functions.inner {
        compiler.compile_function(name.clone(), args.clone(), syntax.clone());
    }

    let entry = code.functions.get(entry).cloned().unwrap();
    Program::new(script.functions, source_map, code, entry)
}

struct Compiler<'r> {
    functions: &'r BTreeSet<String>,
    code: &'r mut Code,
    source_map: &'r mut SourceMap,
}

impl Compiler<'_> {
    fn compile_function(
        &mut self,
        name: String,
        args: Vec<String>,
        syntax: Vec<Expression>,
    ) {
        let mut bindings = args.iter().cloned().collect();
        let mut output = runtime::Function::new(args);

        for expression in syntax {
            self.compile_expression(expression, &mut bindings, &mut output);
        }

        self.code.functions.insert(name, output);
    }

    fn compile_expression(
        &mut self,
        expression: Expression,
        bindings: &mut BTreeSet<String>,
        output: &mut runtime::Function,
    ) {
        match expression.kind {
            ExpressionKind::Binding { names } => {
                for name in names.iter().cloned().rev() {
                    // Inserting bindings unconditionally like that does mean
                    // that bindings can overwrite previously defined bindings.
                    // This is undesirable, but it'll do for now.
                    bindings.insert(name);
                }

                self.generate(
                    Instruction::BindingDefine { name: names },
                    expression.location.clone(),
                    output,
                );
            }
            ExpressionKind::Comment { .. } => {}
            ExpressionKind::Value(value) => {
                self.generate(
                    Instruction::Push { value },
                    expression.location,
                    output,
                );
            }
            ExpressionKind::Word { name } => {
                let instruction =
                    word_to_instruction(name, bindings, self.functions);
                self.generate(instruction, expression.location, output);
            }
        };
    }

    fn generate(
        &mut self,
        instruction: Instruction,
        location: Location,
        output: &mut runtime::Function,
    ) {
        let address = self.code.push(instruction);
        self.source_map.define_mapping(address, location);
        output.instructions.push_back(address);
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
