use capi_process::{Bytecode, Function, FunctionInstructions, Instruction};

use crate::{
    repr::fragments::{
        Fragment, FragmentExpression, FragmentId, FragmentMap, FragmentPayload,
        Fragments,
    },
    source_map::SourceMap,
};

pub fn fragments_to_bytecode(
    mut fragments: Fragments,
) -> (Bytecode, SourceMap) {
    let mut bytecode = Bytecode::default();
    let mut source_map = SourceMap::default();

    let mut compiler = Compiler {
        bytecode: &mut bytecode,
        source_map: &mut source_map,
    };

    for function in fragments.by_function {
        compiler.compile_function(
            function.name,
            function.args,
            function.start,
            &mut fragments.inner,
        );
    }

    dbg!(&bytecode);

    (bytecode, source_map)
}

struct Compiler<'r> {
    bytecode: &'r mut Bytecode,
    source_map: &'r mut SourceMap,
}

impl Compiler<'_> {
    fn compile_function(
        &mut self,
        name: String,
        arguments: Vec<String>,
        start: FragmentId,
        fragments: &mut FragmentMap,
    ) {
        let mut instructions = FunctionInstructions {
            first: None,
            count: 0,
        };

        for fragment in fragments.drain_from(start) {
            self.compile_fragment(fragment, &mut instructions);
        }

        self.bytecode.functions.insert(
            name.clone(),
            Function {
                name,
                arguments,
                instructions,
            },
        );
    }

    fn compile_fragment(
        &mut self,
        fragment: Fragment,
        instructions: &mut FunctionInstructions,
    ) {
        let fragment_id = fragment.id();

        let expression = match fragment.payload {
            FragmentPayload::Expression { expression, .. } => expression,
            FragmentPayload::Terminator => {
                // A terminator only has meaning in the source code
                // representation, but doesn't do anything at runtime.
                return;
            }
        };

        match expression {
            FragmentExpression::BindingDefinitions { names } => {
                self.generate(
                    Instruction::BindingsDefine { names },
                    fragment_id,
                    instructions,
                );
            }
            FragmentExpression::BindingEvaluation { name } => {
                self.generate(
                    Instruction::BindingEvaluate { name },
                    fragment_id,
                    instructions,
                );
            }
            FragmentExpression::BuiltinCall { name } => {
                let instruction = {
                    // Here we check for special built-in functions that are
                    // implemented differently, without making sure anywhere,
                    // that their name doesn't conflict with any user-defined
                    // functions.
                    //
                    // I think it's fine for now. This seems like a temporary
                    // hack anyway, while the language is not powerful enough
                    // to support real conditionals.
                    if name == "return_if_non_zero" {
                        Instruction::ReturnIfNonZero
                    } else if name == "return_if_zero" {
                        Instruction::ReturnIfZero
                    } else {
                        Instruction::CallBuiltin { name }
                    }
                };
                self.generate(instruction, fragment_id, instructions);
            }
            FragmentExpression::Comment { .. } => {}
            FragmentExpression::FunctionCall { name } => {
                self.generate(
                    Instruction::CallFunction { name },
                    fragment_id,
                    instructions,
                );
            }
            FragmentExpression::Value(value) => {
                self.generate(
                    Instruction::Push { value },
                    fragment_id,
                    instructions,
                );
            }
        };
    }

    fn generate(
        &mut self,
        instruction: Instruction,
        fragment_id: FragmentId,
        instructions: &mut FunctionInstructions,
    ) {
        let instruction = self.bytecode.instructions.push(instruction.clone());

        instructions.first = instructions.first.or(Some(instruction));
        instructions.count += 1;

        self.source_map.define_mapping(instruction, fragment_id);
    }
}
